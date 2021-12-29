use crate::config::Config;
use crate::workspace::Workspace;

use clap::{App, AppSettings, Arg, ArgMatches, ArgSettings, PossibleValue};

/// Runs clap
pub fn handle_matches(config: Config) {
    match get_matches(&config).subcommand() {
        Some(("go", sc_matches)) => {
            let workspace = sc_matches.value_of("workspace").unwrap();
            let index = sc_matches.value_of("index").unwrap().to_string();

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
            let new = sc_matches.value_of("new").unwrap();
            super::handle_new_command(new, config);
        }
        Some(("default", _)) => {
            println!("{}", Workspace::from(config).get_name());
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
