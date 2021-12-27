use clap::{App, AppSettings, Arg, ArgSettings, ArgMatches, PossibleValue};

use crate::commands;
use crate::config::Config;
use crate::workspace::{WorkspaceName};

fn handle_main_command(index: String, config: Config) {
    // i3ws:{1}:1:plain
    let mut ws_command = WorkspaceName::with(config);
    ws_command.main_index = index;

    let workspaces = commands::filter_workspaces(|_, wsn|  {
        wsn.main_index == ws_command.main_index && wsn.sub_index == ws_command.sub_index
    });

    // if there is an exact match for the main and sub index
    if let Some(workspaces) = workspaces {
        // use the suffix from the existing workspace
        ws_command.suffix = workspaces[0].suffix.clone();
        commands::run_workspace_command(ws_command);
    }
    else {
        // There is no i3ws{1}-1:xxx maybe there is i3ws{1}-2:xxx
        let workspaces = commands::filter_workspaces(|_, wsn| wsn.main_index == ws_command.main_index);

        if let Some(workspaces) = workspaces {
            // Use the sub workspace and suffix from the existing workspace
            ws_command.sub_index = workspaces[0].sub_index.to_string();
            ws_command.suffix = workspaces[0].suffix.to_string();

            commands::run_workspace_command(ws_command);
        }
        // else workspace doesn't exist yet, make default workspace
        else {
            commands::run_workspace_command(ws_command);
        }
    }
}

fn handle_sub_command(index: String, config: Config) {
    if let Some(mut focused) = commands::get_focused_workspace(&config) {
        // Use the main focused workspace with the give sub workspace
        focused.sub_index = index.to_string();

        // Find if the target workspace exists
        let target_ws = commands::filter_workspaces(|ws, _| ws.name == WorkspaceName::format(&focused));

        let growable = match config.get_type_by_name(focused.suffix.to_string()) {
            Some(n) => n.growable,
            None => false
        };

        // If the target workspace exists or is growable
        if target_ws.is_some() || growable {
            commands::run_workspace_command(focused);
        }
    }
}

fn handle_new_command(_new_type: &str, config: Config) {
    if let Some(_focused) = commands::get_focused_workspace(&config) {
        // Check if the focused workspace and all sub workspaces are empty
    }
}

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
        },
        Some(("new", sc_matches)) => {
            let new = sc_matches.value_of("new").unwrap();
            handle_new_command(new, config);
        },
        Some(("default", _)) => {
            println!("{}", WorkspaceName::format(&WorkspaceName::with(config)));
        }
        _ => {
            panic!("Unknown command");
        }
    }

}

pub fn get_matches(config: &Config) -> ArgMatches {
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
                        .setting(ArgSettings::Required)
                )
                .arg(
                    Arg::new("index")
                        .takes_value(true)
                        .possible_values(["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"])
                        .default_value("0")
                    )
                .setting(AppSettings::ArgRequiredElseHelp)
        )
        .subcommand(
            App::new("new")
                .short_flag('n')
                .about("Creates a new preset workspace")
                .arg(
                    Arg::new("new")
                        .takes_value(true)
                        .possible_values(config.get_type_names().iter().map(|(_, t)| PossibleValue::new(t.as_str())).collect::<Vec<PossibleValue>>())
                        .setting(ArgSettings::Required)
                )
        )

        .subcommand(
            App::new("default")
                .short_flag('d')

                .about("Prints the name of the default workspace")
        )
        .get_matches()
}
