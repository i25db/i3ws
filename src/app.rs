mod matches;
pub use matches::handle_matches;

use crate::commands::*;
use crate::config::Config;
use crate::workspace::Workspace;

/// Handle [main] subcommand
/// Starts by looking for [prefix]:[main]:1: and if it exists
/// activates the workspace. Otherwise, find a workspace that
/// shares the same [main] index. If it can't find that either
/// create a new default workspace.
/// # Arguments
/// - `index` - the name of the main workspace
/// - `config` - the configuration for the whole app
fn handle_main_command(index: String, config: Config) {
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
        run_workspace_command(workspace);
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

            run_workspace_command(workspace);
        }
        // else workspace doesn't exist yet, make default workspace
        else {
            run_workspace_command(workspace);
        }
    }
}

/// Handle [sub] subcommand
/// Looks for [prefix]:[focused]:[index]:. If it exists or
/// is part of a growable workspace activate it.
/// # Arguments
/// - `index` - the name of the sub workspace
/// - `config` - the configuration for the whole app
fn handle_sub_command(index: String, config: Config) {
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
            run_workspace_command(focused);
        }
    }
}

/// Handle [new] subcommand
/// If `new_type` is defined in `config` then find the
/// focused workspace and if its empty create a new
/// workspace. Once all startup commands are executed
/// activates the default sub workspace.
/// # Arguments
/// - `new_type` - the type of new workspace to create.
/// Should be one of `config.get_type_names()`
/// - `config` - the configuration for the whole app
fn handle_new_command(new_type: &str, config: Config) {
    if let Some(mut focused) = query_first(&Query {
        focused: Some(true),
        ..Default::default()
    }) {
        if is_workspace_empty(focused.get_name()) {
            if let Some(ws_type) = config.get_type_by_name(&new_type) {
                focused.suffix = new_type.to_string();

                for commands in &ws_type.ws_commands {
                    for command in &commands.commands {
                        println!("{}", command);
                    }
                }

                focused.sub_index = ws_type.default_ws.to_string();
                run_workspace_command(focused);
            }
        }
    }
}

fn handle_swap_command(index: String, config: Config) {}
