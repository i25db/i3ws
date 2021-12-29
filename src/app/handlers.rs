use crate::commands::*;
use crate::config::Config;
use crate::workspace::Workspace;

pub fn handle_main_command(index: String, config: Config) {
    let mut workspace = Workspace::from(config);
    workspace.main_index = index;

    let q = query_first(&Query {
        main_index: Some(&workspace.main_index),
        sub_index: Some(&workspace.sub_index),
        ..Default::default()
    });
    // if the default workspace exists activate it
    if let Some(ws) = q {
        workspace.suffix = ws.suffix;
        activate_workspace(&workspace.get_name());
    } else {
        // There is no [prefix]:[main]:1, check for [prefix]:[main]:(2..0)
        let workspaces = query_first(&Query {
            main_index: Some(&workspace.main_index),
            ..Default::default()
        });

        if let Some(ws) = workspaces {
            // Use the sub workspace and suffix from the existing workspace
            workspace.sub_index = ws.sub_index;
            workspace.suffix = ws.suffix;

            activate_workspace(&workspace.get_name());
        }
        // else workspace doesn't exist yet, make default workspace
        else {
            activate_workspace(&workspace.get_name());
        }
    }
}

pub fn handle_sub_command(index: String, config: Config) {
    if let Some(mut focused) = query_first(&Query {
        focused: Some(true),
        ..Default::default()
    }) {
        focused.sub_index = index;

        let target = query(&Query {
            main_index: Some(&focused.main_index),
            sub_index: Some(&focused.sub_index),
            ..Default::default()
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
    if let Some(mut focused) = query_first(&Query {
        focused: Some(true),
        ..Default::default()
    }) {
        if is_workspace_empty(focused.get_name()) {
            if let Some(ws_type) = config.get_type_by_name(&new_type) {
                focused.suffix = new_type;

                for commands in &ws_type.ws_commands {
                    for command in &commands.commands {
                        println!("{}", command);
                    }
                }

                focused.sub_index = ws_type.default_ws.to_string();
                activate_workspace(&focused.get_name());
            }
        }
    }
}

pub fn handle_swap_command(index: String, config: Config) {
    // 1) Check if [index]:* exists
    //  a. If it does copy all [index]:*:prefix ->  i3wsswap:[index]:*:prefix
    // 2) Copy all [focused]:* -> [index]:*
    // 3) Copy all i3wssap -> [prefix]:[focused]:*:*
}
