use clap::{App, AppSettings, Arg, ArgSettings, ArgMatches};

pub const PREFIX: &str = "i3ws";
pub const PLAIN_SUFFIX: &str = ":";
pub const CODE_SUFFIX: &str = ":code";
pub const GAME_SUFFIX: &str = ":game";

#[derive(Debug)]
pub struct WorkspaceName {
    pub prefix: String,
    pub main_index: String,
    pub sub_index: String,
    pub suffix: String
}

impl Default for WorkspaceName {
    fn default() -> Self {
        Self {
            prefix: String::from(PREFIX),
            main_index: String::from("1"),
            sub_index: String::from("1"),
            suffix: String::from(PLAIN_SUFFIX)
        }
    }
}

impl From<&WorkspaceName> for WorkspaceName {
    fn from(name: &WorkspaceName) -> Self {
        Self {
            prefix: String::from(&name.prefix),
            main_index: String::from(&name.main_index),
            sub_index: String::from(&name.sub_index),
            suffix: String::from(&name.suffix)
        }
    }
}

impl From<WorkspaceName> for String {
    fn from(name: WorkspaceName) -> Self {
        format(&name)
    }
}

pub fn format(name: &WorkspaceName) -> String {
    format!("{}{}-{}{}", name.prefix, name.main_index, name.sub_index, name.suffix)
}

impl Into<WorkspaceName> for String {
    fn into(self) -> WorkspaceName {
        if !self.contains(PREFIX) || !self.contains(':') || !self.contains('-') {
            return WorkspaceName::default();
        }

        let name = self.trim_start_matches(PREFIX);

        let main_index = String::from(&name[0..1]);
        let sub_index = String::from(&name[2..3]);
        let suffix = String::from(&name[name.find(':').unwrap()..]);

        WorkspaceName {
            prefix: String::from(PREFIX),
            main_index,
            sub_index,
            suffix
        }
    }
}

impl Into<WorkspaceName> for &String {
    fn into(self) -> WorkspaceName {
        if !self.contains(PREFIX) || !self.contains(':') || !self.contains('-') {
            return WorkspaceName::default();
        }

        let name = self.trim_start_matches(PREFIX);
        let main_index = String::from(&name[0..1]);
        let sub_index = String::from(&name[2..3]);
        let suffix = String::from(&name[name.find(':').unwrap()..]);

        WorkspaceName {
            prefix: String::from(PREFIX),
            main_index,
            sub_index,
            suffix
        }
    }
}

use crate::json;
use crate::commands;

fn filter_workspaces<F>(workspaces: &Vec<json::Workspace>, f: F) -> Vec<WorkspaceName> where F: Fn(&&json::Workspace, WorkspaceName) -> bool {
    workspaces.iter()
        .filter(|ws| {
            let wsn: WorkspaceName = (&ws.name).into();
            f(ws, wsn)
        })
        .map(|ws| {
            let ws: WorkspaceName = (&ws.name).into();
            ws
        }).collect::<Vec<WorkspaceName>>()
}

fn handle_main_command(index: String) {
    let mut ws_command = WorkspaceName { main_index: index, ..Default::default() };

    let json_workspaces = json::parse_workspaces(&commands::get_workspaces());
    let workspaces = filter_workspaces(&json_workspaces, 
        |_, wsn| wsn.main_index == ws_command.main_index && wsn.sub_index == ws_command.sub_index);

    // if there is an exact match for the main and sub index
    if workspaces.len() == 1 {
        println!("Found exact match for main workspace: {:?}", workspaces[0]);
        // use the suffix from the existing workspace
        ws_command.suffix = workspaces[0].suffix.to_string();
        commands::run_workspace_command(ws_command);
    }
    else {
        let workspaces = filter_workspaces(&json_workspaces, 
            |_, wsn| wsn.main_index == ws_command.main_index);

        // If there is an existing main workspace but a nondefault sub workspace
        if workspaces.len() > 0 {
            println!("Found nondefault sub workspace for: {:?}", workspaces[0]);
            // Use the sub workspace and suffix from the existing workspace
            ws_command.sub_index = workspaces[0].sub_index.to_string();
            ws_command.suffix = workspaces[0].suffix.to_string();

            commands::run_workspace_command(ws_command);
        }
        // else workspace doesn't exist yet, make default workspace
        else {
            println!("Main workspace doesn't exist yet: {:?}", ws_command);
            commands::run_workspace_command(ws_command);
        }
    }
}

fn handle_sub_command(index: String) {
    let json_workspaces = json::parse_workspaces(&commands::get_workspaces());

    // Find a focused i3ws workspace
    let workspaces = filter_workspaces(&json_workspaces,
        |ws, _| ws.focused && ws.name.starts_with(PREFIX));

    if workspaces.len() > 0 {
        let focused_name = &workspaces[0];
        println!("Found focused workspace: {:?}", &focused_name);

        let mut target_ws_name = WorkspaceName::from(focused_name);
        target_ws_name.sub_index = index;

        // Find if the target workspace exists
        let workspaces = filter_workspaces(&json_workspaces, 
            |ws, _| ws.name == format(&target_ws_name));

        // If the target workspace doesn't exist but its in a plain main workspace
        if workspaces.len() == 0 && focused_name.suffix == PLAIN_SUFFIX.to_string() {
            println!("Target workspace doesn't exist: {:?}", target_ws_name);
            commands::run_workspace_command(target_ws_name);
        }
        // If the target workspace does exist go there
        else if workspaces.len() == 1 {
            println!("Target workspace exists: {:?}", target_ws_name);
            commands::run_workspace_command(target_ws_name);
        }
    }
}

pub fn handle_matches(matches: ArgMatches) {
    match matches.subcommand() {
        Some(("go", sc_matches)) => {
            let workspace = sc_matches.value_of("workspace").unwrap();
            let index = sc_matches.value_of("index").unwrap().to_string();

            match workspace {
                "main" => {
                    handle_main_command(index);
                }
                "sub" => {
                    handle_sub_command(index);
                }
                _ => {
                    panic!("Invalid workspace");
                }
            }
        },
        Some(("new", sc_matches)) => {
            let new = sc_matches.value_of("new").unwrap();
        }
        _ => {
            panic!("Unknown command");
        }
    }

}

pub fn get_matches() -> ArgMatches {
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
                        .possible_values(&["plain", "code", "game"])
                        .setting(ArgSettings::Required)
                )
        )
        .get_matches()
}
