use std::cmp::Ordering;


use crate::commands::*;
use crate::config::Config;
use crate::workspace::Workspace;

pub fn handle_main_command(index: u32, config: Config) {
    let mut main_ws = Workspace::from(config);
    main_ws.main_index = index;

    // if the default workspace exists activate it
    if let Some(ws) = query_first(|ws| {
        &ws.main_index == &main_ws.main_index && &ws.sub_index == &main_ws.sub_index
    }) {
        main_ws.suffix = ws.suffix;
        activate_workspace(&main_ws.get_name());
    } else {
        // There is no [prefix]:[main]:1, check for [prefix]:[main]:(2..0)
        let workspaces = query_first(|ws| &ws.main_index == &main_ws.main_index);

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

    if let Some(mut focused) = query_first(|ws| ws.focused) {
        focused.sub_index = index;

        let target = query(|ws| {
            &ws.main_index == &focused.main_index && &ws.sub_index == &focused.sub_index
        });

        let growable = match config.get_type_by_name(&focused.suffix) {
            Some(n) => n.growable,
            None => false,
        };

        // If the target workspace exists or is growable
        if target.is_some() || growable {
            activate_workspace(&focused.get_name());
        }
    }
}

pub fn handle_new_command(new_type: String, config: Config) {
    if let Some(mut focused) = query_first(|ws| ws.focused) {
        if is_workspace_empty(focused.get_name()) {
            if let Some(ws_type) = config.get_type_by_name(&new_type) {
                focused.suffix = new_type;

                for (_ws, _command) in &ws_type.commands {}

                focused.sub_index = ws_type.default_sub_workspace;
                activate_workspace(&focused.get_name());
            }
        }
    }
}

pub fn handle_sub_swap_command(index: u32, config: Config) {
    // 1) Check if *:[focused]:[index]:* exists
    //  a. If it does move it to [swap_prefix]:[focused]:[index]:*
    if let Some(focused) = query_first(|ws| ws.focused) {
        if focused.prefix == config.default_prefix && config.swap_on_default_only {
            if let Some(dest) =
                query_first(|ws| &ws.main_index == &focused.main_index && &ws.sub_index == &index)
            {
                let mut tmp = dest.clone();
                tmp.prefix = config.default_swap_prefix.clone();

                move_workspace(&dest.get_name(), &tmp.get_name(), false);
            }

            // 2) Copy *:[focused]:[focused]:* -> *:[focused]:[index]:*
            let mut tmp = focused.clone();
            tmp.sub_index = index;


            move_workspace(&focused.get_name(), &tmp.get_name(), config.swap_on_sub);

            // 3) Copy [swap_prefix]:*:*:* -> *:[focused]:[focused]:*
            if let Some(swap) = query_first(|ws| &ws.prefix == &config.default_swap_prefix) {
                let mut tmp = swap.clone();
                tmp.prefix = config.default_prefix;

                tmp.sub_index = focused.sub_index;

                move_workspace(&swap.get_name(), &tmp.get_name(), !config.swap_on_sub);
            }
        }
    }
}

pub fn handle_main_swap_command(index: u32, config: Config) {

    // 1) Check if *:*:[index]:* exists
    //  a. If it does copy all *:[index]:*:* ->  [swap_prefix]:[index]:*:*
    if let Some(dest) = query(|ws| &ws.main_index == &index) {
        // println!("Found {} in destination", dest[0].get_name());
        for ws in &dest {
            let mut tmp = ws.clone();
            tmp.prefix = config.default_swap_prefix.clone();

            // println!("  Moving {} to {}", &ws.get_name(), &tmp.get_name());
            move_workspace(&ws.get_name(), &tmp.get_name(), false);
        }
    }
    // 2) Copy all [default_prefix]:[focused]:* -> [index]:*
    if let Some(focused) = query_first(|ws| ws.focused) {
        let origin_main_index = focused.main_index;
        let origin_sub_index = focused.sub_index;


        if let Some(focused) = query(|ws| {
            &ws.main_index == &focused.main_index && &ws.prefix == &config.default_prefix
        }) {
            // println!("Copying {} to destination", focused[0].get_name());
            for ws in &focused {
                let mut tmp = ws.clone();
                tmp.main_index = index;


                // println!("  Copying {} to {}", &ws.get_name(), &tmp.get_name());

                move_workspace(
                    &ws.get_name(),
                    &tmp.get_name(),
                    ws.sub_index == origin_sub_index && config.swap_on_main,
                );
            }
        }

        // 3) Copy all i3wsswap -> [prefix]:[focused]:*:*
        if let Some(swaps) = query(|ws| &ws.prefix == &config.default_swap_prefix) {
            // println!("Swapping {} to origin", swaps[0].get_name());
            for swap in &swaps {
                let mut tmp = swap.clone();
                tmp.prefix = config.default_prefix.clone();
                tmp.main_index = origin_main_index;


                // println!("  Swapping {} to {}", &swap.get_name(), &tmp.get_name());

                // TODO: if swap_on_main go back to original sub workspace
                move_workspace(&swap.get_name(), &tmp.get_name(), !config.swap_on_main);
            }
        }
    }
}

pub fn handle_info_command(t: &str, config: Config) {
    match t {
        "current" => {
            let current = query_first(|ws| ws.focused).expect("Found no focused workspace");

            let format = &config
                .get_type_by_name(&current.suffix)
                .expect("Current workspace does not have a type")
                .display_name;
            println!(
                "{}",
                format.replace("{index}", &current.main_index.to_string())
            );
        }
        "all_mains" => {
            let focused_index = query_first(|ws| ws.focused)
                .expect("Found no focused workspace")
                .main_index;

            let mut main_indexes: Vec<u32> = Vec::new();
            let mut workspaces = query(|ws| {
                if main_indexes.contains(&ws.main_index) {
                    return false;
                }

                main_indexes.push(ws.main_index);
                return true;
            })
            .expect("Found no workspaces");
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
                let format = &config
                    .get_type_by_name(&ws.suffix)
                    .expect("Current workspace does not have a type")
                    .display_name;
                let format = format.replace("{index}", &ws.main_index.to_string());

                if first {
                    print!("{}", format)
                } else {
                    print!("|{}", format);
                }

                first = false;
            }
            println!();
        }
        "all_subs" => {
            let current = query_first(|ws| ws.focused).expect("Found no focused workspace");
            let mut subs: Vec<Workspace> = query(|ws| {
                println!("{:?}\n{:?} \nPrefix: {} \nIndex: {}\n", ws, current, config.default_prefix, ws.main_index == current.main_index);
                &ws.prefix == &config.default_prefix && ws.main_index == current.main_index
            })
            .unwrap_or(Vec::<Workspace>::new());

            println!("{:?}", subs);

            subs.sort_by(|a, b| a.sub_index.cmp(&b.sub_index));

            let mut first = true;

            for sub in &subs {
                let format = &config
                    .get_type_by_name(&sub.suffix)
                    .expect("Current sub workspace does not have a type")
                    .sub_display_name;
                let mut format = format.replace("{index}", &sub.sub_index.to_string());

                if sub.focused {
                    format = format!("**{}", format);
                }

                if first {
                    print!("{}", format)
                } else {
                    print!("|{}", format);
                }

                first = false;
            }

            println!();
        }
        t => {
            panic!("Unknown info type: {}", t);
        }
    }
}
