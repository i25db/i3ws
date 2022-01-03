use crate::config::Config;
use crate::workspace::Workspace;
use crate::{check_error, check_some, safe_panic};

use clap::{App, AppSettings, Arg, ArgMatches, ArgSettings, PossibleValue};

/// Runs clap
pub fn handle_matches(config: Config) {
    match get_matches(&config).subcommand() {
        Some(("go", sc_matches)) => {
            let workspace = check_some!(sc_matches.value_of("workspace"), "No workspace given");
            let index = check_error!(
                check_some!(sc_matches.value_of("index"), "No index given").parse::<u32>(),
                "Index should be a number: {}"
            );

            match workspace {
                "main" => {
                    super::handle_main_command(index, config);
                }
                "sub" => {
                    super::handle_sub_command(index, config);
                }
                _ => {
                    panic!("Invalid workspace");
                }
            }
        }
        Some(("new", sc_matches)) => {
            let new = check_some!(sc_matches.value_of("new"), "No type given").to_string();
            super::handle_new_command(new, config);
        }
        Some(("swap", sc_matches)) => {
            let index = check_error!(
                check_some!(sc_matches.value_of("index"), "No index given").parse::<u32>(),
                "Index should be a number: {}"
            );
            let dest = check_some!(sc_matches.value_of("dest"), "No destination given");
            
            match dest {
                "sub" => {
                    super::handle_sub_swap_command(index, config);
                }
                "main" => {
                    super::handle_main_swap_command(index, config);
                }
                dest => safe_panic!("Unknown destination: {}", dest),
            };
        }
        Some(("info", sc_matches)) => {
            let t = sc_matches.value_of("type").unwrap();

            super::handle_info_command(t, config);
        }
        Some(("default", _)) => {
            crate::commands::activate_workspace(&Workspace::from(config).get_name());
        }
        _ => {
            safe_panic!("Unknown command");
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
                        .possible_values(["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"]),
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
            App::new("swap")
                .short_flag('s')
                .about("Swaps the current workspace with the given workspace")
                .arg(
                    Arg::new("dest")
                        .short('d')
                        .takes_value(true)
                        .possible_values(["main", "sub"])
                        .default_value("sub"),
                )
                .arg(
                    Arg::new("index")
                        .takes_value(true)
                        .possible_values(["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"])
                        .default_value("0"),
                ),
        )
        .subcommand(
            App::new("default")
                .short_flag('d')
                .about("Prints the name of the default workspace"),
        )
        .subcommand(
            App::new("info")
                .about("Prints info about the current workspaces")
                .arg(
                    Arg::new("type")
                        .takes_value(true)
                        .possible_values(["current", "all_mains", "all_subs"])
                        .default_value("current"),
                ),
        )
        .get_matches()
}
