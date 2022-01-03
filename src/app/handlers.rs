use std::cmp::Ordering;

use crate::commands::*;
use crate::config::Config;
use crate::workspace::Workspace;
use crate::{check_some, safe_panic};

pub fn handle_main_command(index: u32, config: Config) {
    let mut main_ws = Workspace::from(config);
    main_ws.main_index = index;

    // if the default workspace exists activate it
    if let Some(ws) = query_index(&main_ws.main_index, &main_ws.sub_index) {
        main_ws.suffix = ws.suffix;
        activate_workspace(&main_ws.get_name());
    } else {
        // There is no [prefix]:[main]:1, check for [prefix]:[main]:(2..0)
        let workspaces = query_main(&main_ws.main_index);

        if let Some(sub_ws) = workspaces {
            // Use the sub workspace and suffix from the existing workspace
            main_ws.sub_index = sub_ws.sub_index;
            main_ws.suffix = sub_ws.suffix;

            activate_workspace(&main_ws.get_name());
        }
        // else workspace doesn't exist yet, make default workspace
        else {
            activate_workspace(&main_ws.get_name());
        }
    }
}

pub fn handle_sub_command(index: u32, config: Config) {
    let mut focused = query_focused();
    focused.sub_index = index;

    let t = &config.get_type_by_name(&focused.suffix);
    if let Some(max_subs) = t.max_sub_count {
        if index > max_subs {
            return;
        }
    }

    let target = query_index(&focused.main_index, &focused.sub_index);

    let growable = &config.get_type_by_name(&focused.suffix).growable;

    // If the target workspace exists or is growable
    if target.is_some() || *growable {
        activate_workspace(&focused.get_name());
    }
}

pub fn handle_new_command(new_type: String, config: Config) {
    let mut focused = query_focused();
    if is_workspace_empty(focused.get_name()) {
        let ws_type = &config.get_type_by_name(&new_type);
        focused.suffix = new_type;

        for (_ws, _command) in &ws_type.commands {}

        focused.sub_index = ws_type.default_sub_workspace;
        activate_workspace(&focused.get_name());
    }
}

pub fn handle_sub_swap_command(index: u32, config: Config) {
    // 1) Check if *:[focused]:[index]:* exists
    //  a. If it does move it to [swap_prefix]:[focused]:[index]:*
    let focused = query_focused();
    // Don't swap to a workspace that is out of bounds
    let t = &config.get_type_by_name(&focused.suffix);
    if let Some(max_subs) = t.max_sub_count {
        if index > max_subs {
            return;
        }
    }

    if focused.prefix == config.default_prefix && config.swap_on_default_only {
        if let Some(dest) = query_index(&focused.main_index, &index) {
            let mut tmp = dest.clone();
            tmp.prefix = config.default_swap_prefix.clone();

            move_workspace(&dest.get_name(), &tmp.get_name(), false);
        }

        // 2) Copy *:[focused]:[focused]:* -> *:[focused]:[index]:*
        let mut tmp = focused.clone();
        tmp.sub_index = index;

        move_workspace(&focused.get_name(), &tmp.get_name(), config.swap_on_sub);

        // 3) Copy [swap_prefix]:*:*:* -> *:[focused]:[focused]:*
        if let Some(swap) = query_swap(&config) {
            let mut tmp = swap.clone();
            tmp.prefix = config.default_prefix;

            tmp.sub_index = focused.sub_index;

            move_workspace(&swap.get_name(), &tmp.get_name(), !config.swap_on_sub);
        }
    }
}

pub fn handle_main_swap_command(index: u32, config: Config) {
    // 1) Check if *:*:[index]:* exists
    //  a. If it does copy all *:[index]:*:* ->  [swap_prefix]:[index]:*:*
    if let Some(dest) = query_all_by_main(&index) {
        for ws in &dest {
            let mut tmp = ws.clone();
            tmp.prefix = config.default_swap_prefix.clone();

            move_workspace(&ws.get_name(), &tmp.get_name(), false);
        }
    }
    // 2) Copy all [default_prefix]:[focused]:* -> [index]:*
    let focused = query_focused();
    let origin_main_index = focused.main_index;
    let origin_sub_index = focused.sub_index;

    if let Some(focused) = query_all_by_main(&focused.main_index) {
        for ws in &focused {
            let mut tmp = ws.clone();
            tmp.main_index = index;

            move_workspace(
                &ws.get_name(),
                &tmp.get_name(),
                ws.sub_index == origin_sub_index && config.swap_on_main,
            );
        }
    }

    // 3) Copy all i3wsswap -> [prefix]:[focused]:*:*
    if let Some(swaps) = query_all_swaps(&config) {
        for swap in &swaps {
            let mut tmp = swap.clone();
            tmp.prefix = config.default_prefix.clone();
            tmp.main_index = origin_main_index;

            // TODO: if swap_on_main go back to original sub workspace
            move_workspace(&swap.get_name(), &tmp.get_name(), !config.swap_on_main);
        }
    }
}

pub fn handle_info_command(t: &str, config: Config) {
    match t {
        "current" => {
            let current = query_focused();

            let format = &config
                .get_type_by_name(&current.suffix)
                .display_name_focused;
            println!(
                "{}",
                format.replace("{index}", &current.main_index.to_string())
            );
        }
        "all_mains" => {
            let focused_index = query_focused().main_index;

            let mut main_indexes: Vec<u32> = Vec::new();
            let mut workspaces = check_some!(
                query(|ws| {
                    if main_indexes.contains(&ws.main_index) {
                        return false;
                    }

                    main_indexes.push(ws.main_index);
                    return true;
                }),
                "Found no workspaces"
            );
            workspaces.sort_by(|a, b| {
                if a.main_index == focused_index {
                    Ordering::Less
                } else if b.main_index == focused_index {
                    Ordering::Greater
                } else {
                    a.main_index.cmp(&b.main_index)
                }
            });

            let mut first = true;
            for ws in &workspaces {
                let t = &config.get_type_by_name(&ws.suffix);

                let format = if ws.focused {
                    &t.display_name_focused
                } else {
                    &t.display_name
                };

                let format = format.replace("{index}", &ws.main_index.to_string());

                if first {
                    print!("{}", format)
                } else {
                    print!("{}{}", t.display_sep, format);
                }

                first = false;
            }
            println!();
        }
        "all_subs" => {
            let current = query_focused();
            let mut subs: Vec<Workspace> = check_some!(
                query_all_by_main(&current.main_index),
                "No sub workspaces found"
            );
            subs.sort_by(|a, b| a.sub_index.cmp(&b.sub_index));

            let t = &config.get_type_by_name(&subs[0].suffix);

            let mut formats: Vec<String> = subs
                .iter()
                .map(|ws| {
                    if ws.focused {
                        t.sub_display_name_focused.clone()
                    } else {
                        t.sub_display_name.clone()
                    }
                })
                .collect();

            if let Some(max_subs) = t.max_sub_count {
                formats = Vec::new();
                while formats.len() < max_subs as usize {
                    formats.push(t.display_name_empty.clone());
                }

                for ws in subs.iter().filter(|ws| ws.focused) {
                    formats[(ws.sub_index - 1) as usize] = t
                        .sub_display_name_focused
                        .replace("{index}", &ws.sub_index.to_string())
                }
            }

            let mut first = true;

            for format in formats.iter() {
                if first {
                    print!("{}", format)
                } else {
                    print!("{}{}", t.display_sep, format);
                }

                first = false;
            }

            println!();
        }
        t => {
            safe_panic!("Unknown info type: {}", t);
        }
    }
}
