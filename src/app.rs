use clap::{App, AppSettings, Arg, ArgMatches, ArgSettings, PossibleValue};

use crate::commands;
use crate::config::Config;
use crate::workspace::WorkspaceName;

/// Handle [main] subcommand
/// Starts by looking for [prefix]:[main]:1: and if it exists
/// activates the workspace. Otherwise, find a workspace that
/// shares the same [main] index. If it can't find that either
/// create a new default workspace.
/// # Arguments
/// - `index` - the name of the main workspace
/// - `config` - the configuration for the whole app
fn handle_main_command(index: String, config: Config) {
    let mut workspace = WorkspaceName::with(config);
    workspace.main_index = index;

    let workspaces = commands::filter_workspaces(|_, wsn| {
        wsn.main_index == workspace.main_index && wsn.sub_index == workspace.sub_index
    });

    // if the default workspace exists activate it
    if let Some(workspaces) = workspaces {
        workspace.suffix = workspaces[0].suffix.clone();
        commands::run_workspace_command(&workspace);
    } else {
        // There is no [prefix]:[main]:1, check for [prefix]:[main]:(2..0)
        let workspaces =
            commands::filter_workspaces(|_, wsn| wsn.main_index == workspace.main_index);

        if let Some(workspaces) = workspaces {
            // Use the sub workspace and suffix from the existing workspace
            workspace.sub_index = workspaces[0].sub_index.to_string();
            workspace.suffix = workspaces[0].suffix.to_string();

            commands::run_workspace_command(&workspace);
        }
        // else workspace doesn't exist yet, make default workspace
        else {
            commands::run_workspace_command(&workspace);
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
    if let Some(mut focused) = commands::get_focused_workspace(&config) {
        focused.sub_index = index.to_string();

        let target = commands::filter_workspaces(|_, wsn| {
            wsn.main_index == focused.main_index && wsn.sub_index == focused.main_index
        });

        let growable = match config.get_type_by_name(focused.suffix.to_string()) {
            Some(n) => n.growable,
            None => false,
        };

        // If the target workspace exists or is growable
        if target.is_some() || growable {
            commands::run_workspace_command(&focused);
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
    if commands::is_focused_workspace_empty(&config) {
        // Make a new workspace
        if let Some(ws_type) = config.get_type_by_name(new_type.to_string()) {
            let mut focused = commands::get_focused_workspace(&config).unwrap();
            focused.suffix = new_type.to_string();

            for commands in &ws_type.ws_commands {
                focused.sub_index = commands.sub_ws.clone();

                // execute all the commands here
                for command in &commands.commands {
                    println!("{}", command);
                    // move container to the proper workspace
                }
            }

            focused.sub_index = ws_type.default_ws.clone();
            commands::run_workspace_command(&focused);
        }
    }
}

/// Runs clap
pub fn handle_matches(config: Config) {
    match get_matches(&config).subcommand() {
        Some(("go", sc_matches)) => {
            let workspace = sc_matches.value_of("workspace").unwrap();
            let index = sc_matches.value_of("index").unwrap().to_string();

            match workspace {
                "main" => {
                    handle_main_command(index, config);
                }
                "sub" => {
                    handle_sub_command(index, config);
                }
                _ => {
                    panic!("Invalid workspace");
                }
            }
        }
        Some(("new", sc_matches)) => {
            let new = sc_matches.value_of("new").unwrap();
            handle_new_command(new, config);
        }
        Some(("default", _)) => {
            println!("{}", WorkspaceName::format(&WorkspaceName::with(config)));
        }
        _ => {
            panic!("Unknown command");
        }
    }
}

fn get_matches(config: &Config) -> ArgMatches {
    App::new("i3ws")
        .author("i25db <i25.db@outlook.com>")
        .version("v0.0.1")
        .about("A CLI tool for managing i3 workspaces")
        .setting(AppSettings::SubcommandRequired)
        .setting(AppSettings::DisableHelpSubcommand)
        .subcommand(
            App::new("go")
                .short_flag('g')
                .about("Go to a workspace")
                .arg(
                    Arg::new("workspace")
                        .takes_value(true)
                        .possible_values(["main", "sub"])
                        .setting(ArgSettings::Required),
                )
                .arg(
                    Arg::new("index")
                        .takes_value(true)
                        .possible_values(["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"])
                        .default_value("0"),
                )
                .setting(AppSettings::ArgRequiredElseHelp),
        )
        .subcommand(
            App::new("new")
                .short_flag('n')
                .about("Creates a new preset workspace")
                .arg(
                    Arg::new("new")
                        .takes_value(true)
                        .possible_values(
                            config
                                .get_type_names()
                                .iter()
                                .map(|(_, t)| PossibleValue::new(t.as_str()))
                                .collect::<Vec<PossibleValue>>(),
                        )
                        .setting(ArgSettings::Required),
                ),
        )
        .subcommand(
            App::new("default")
                .short_flag('d')
                .about("Prints the name of the default workspace"),
        )
        .get_matches()
}
