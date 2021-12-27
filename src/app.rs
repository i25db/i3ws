use clap::{App, AppSettings, Arg, ArgSettings, ArgMatches, PossibleValue};

#[derive(Debug, Clone)]
pub struct WorkspaceName {
    pub prefix: String,
    pub main_index: String,
    pub sub_index: String,
    pub suffix: String
}

impl Default for WorkspaceName {
    fn default() -> Self {
        WorkspaceName::with(Config::default())
    }
}

impl WorkspaceName {
    pub fn with(config: Config) -> Self {
        Self {
            prefix: config.prefix,
            main_index: "1".to_string(),
            sub_index: "1".to_string(),
            suffix: config.default_suffix
        }
    }

    pub fn format(name: &WorkspaceName) -> String {
        format!("{}:{}:{}:{}", name.prefix, name.main_index, name.sub_index, name.suffix)
    }
}

impl From<&WorkspaceName> for String {
    fn from(name: &WorkspaceName) -> Self {
        WorkspaceName::format(name)
    }
}

impl From<&String> for WorkspaceName {
    fn from(name: &String) -> Self {
        let split: Vec<String> = name.split(':').map(|s| s.to_string()).collect();

        if split.len() != 4 {
            panic!("Workspace '{}' can't be parsed", name);
        }

        Self {
            prefix: split[0].to_string(),
            main_index: split[1].to_string(),
            sub_index: split[2].to_string(),
            suffix: split[3].to_string()
        }
    }
}

use crate::json;
use crate::commands;
use crate::config::Config;

fn filter_workspaces<F>(workspaces: &Vec<json::Workspace>, f: F) -> Vec<WorkspaceName> where F: Fn(&&json::Workspace, WorkspaceName) -> bool {
    workspaces.iter()
        .filter(|ws| {
            let wsn: WorkspaceName = WorkspaceName::from(&ws.name);
            f(ws, wsn)
        })
        .map(|ws| {
            WorkspaceName::from(&ws.name)
        }).collect::<Vec<WorkspaceName>>()
}

fn handle_main_command(index: String, config: Config) {
    // i3ws:{1}:1:plain
    let mut ws_command = WorkspaceName::with(config);
    ws_command.main_index = index;

    let json_workspaces = json::parse_workspaces(&commands::get_workspaces());
    let workspaces = filter_workspaces(&json_workspaces,
        |_, wsn| wsn.main_index == ws_command.main_index && wsn.sub_index == ws_command.sub_index);

    // if there is an exact match for the main and sub index
    if workspaces.len() == 1 {
        // use the suffix from the existing workspace
        ws_command.suffix = workspaces[0].suffix.clone();
        commands::run_workspace_command(ws_command);
    }
    else {
        // There is no i3ws{1}-1:xxx maybe there is i3ws{1}-2:xxx
        let workspaces = filter_workspaces(&json_workspaces,
            |_, wsn| wsn.main_index == ws_command.main_index);

        if workspaces.len() > 0 {
            // Use the sub workspace and suffix from the existing workspace
            ws_command.sub_index = workspaces[0].sub_index.clone();
            ws_command.suffix = workspaces[0].suffix.clone();

            commands::run_workspace_command(ws_command);
        }
        // else workspace doesn't exist yet, make default workspace
        else {
            commands::run_workspace_command(ws_command);
        }
    }
}

fn handle_sub_command(index: String, config: Config) {
    let json_workspaces = json::parse_workspaces(&commands::get_workspaces());

    // Find a focused i3ws workspace
    let focused = filter_workspaces(&json_workspaces,
        |ws, _| ws.focused && ws.name.starts_with(config.prefix.as_str()));

    if focused.len() > 0 {
        // Use the main focused workspace with the give sub workspace
        let mut target_main_ws = focused[0].clone();
        target_main_ws.sub_index = index;

        // Find if the target workspace exists
        let target_ws = filter_workspaces(&json_workspaces,
            |ws, _| ws.name == WorkspaceName::format(&target_main_ws));

        let growable = match config.get_type_by_name(target_main_ws.suffix.clone()) {
            Some(n) => n.growable,
            None => false
        };

        println!("Suffix {} is growable {}", target_main_ws.suffix, growable);

        // If the target workspace exists or is growable
        if target_ws.len() == 1 || growable {
            commands::run_workspace_command(target_main_ws.clone());
        }
    }
}

fn handle_new_command(new: &str, config: Config) {
    let json_workspaces = json::parse_workspaces(&commands::get_workspaces());
    let focused = filter_workspaces(&json_workspaces,
        |ws, _| ws.focused && ws.name.starts_with(config.prefix.as_str()));

    let focused_sub_ws = filter_workspaces(&json_workspaces,
        |_, wsn| wsn.main_index == focused[0].main_index);

    if focused_sub_ws.len() == 1 {
        // There is only one sub workspace
        // Check if the workspace is empty
    }

    match new {
        new => {
            if let Some(t) = config.types.iter().find(|t| t.name == new.to_string()) {
                for command in &t.ws_commands {
                    let _sub_ws = &command.sub_ws;
                    for _command in &command.commands {

                    }
                }
            }
        }
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
