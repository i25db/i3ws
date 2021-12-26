use clap::{App, AppSettings, Arg, ArgSettings, ArgMatches};
use std::process::{Command, Output};
use serde::Serialize;
use serde::Deserialize;
use std::str;
use std::io;

const PREFIX: &str = "i3ws";
const PLAIN_SUFFIX: &str  = ":plain";
const CODE_SUFFIX: &str = ":code";
const GAME_SUFFIX: &str = ":game";

fn main() {
    let matches = get_matches();

    match matches.subcommand() {
        Some(("go", sc_matches)) => {
            let should_create = match sc_matches.index_of("create") {
                Some(_) => true,
                _ => false
            };

            let workspace = sc_matches.value_of("workspace").unwrap();
            let index = sc_matches.value_of("index").unwrap();

            match workspace {
                "main" => {
                    run_workspace_command(PREFIX, index, "1", PLAIN_SUFFIX).expect("Error running i3-msg command");
                }
                "sub" => {
                    let output = Command::new("i3-msg")
                        .args(["-t", "get_workspaces"]).output().expect("Failed to execute i3-msg command");
                    let workspaces = parse_workspaces(str::from_utf8(&output.stdout).unwrap());

                    // Find a focused i3ws workspace
                    let mut main_index: Option<char> = None;
                    for ws in &workspaces {
                        println!("{:?}", ws);
                        if ws.focused {
                            let name = &ws.name;

                            // name = i3ws1-1
                            if name.starts_with(PREFIX) {
                                let name = name.trim_start_matches(PREFIX);

                                main_index = Some(name.as_bytes()[0] as char);
                                println!("Found focused main workspace");
                                break;
                            }
                        }
                    }

                    // If there was an existing main workspace
                    if let Some(main_index) = main_index {
                        let sub_name = format!("{}{}-{}", PREFIX, main_index, index);

                        println!("Working on {}", sub_name);

                        // switch to/create sub workspace
                        if should_create {
                            println!("Creating sub workspace {}", sub_name);
                            Command::new("i3-msg")
                                .arg(format!("workspace {}", sub_name))
                                .output().expect("Failed to execute i3-msg command");
                        }
                        // find if the sub workspace exists
                        else {
                            for ws in workspaces {
                                if ws.name == sub_name {
                                    println!("Switching to sub workspace {}", sub_name);
                                    Command::new("i3-msg")
                                        .arg(format!("workspace {}", sub_name))
                                        .output().expect("Failed to execute i3-msg command");
                                    break;
                                }
                            }
                        }
                    }
                }
                _ => {
                    panic!("Invalid workspace");
                }
            }
        },
        Some(("new", _sc_matches)) => {

        }
        _ => {

        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Workspace {
    id: i64,
    num: i32,
    name: String,
    visible: bool,
    focused: bool,
    rect: Rect,
    output: String,
    urgent: bool
}

#[derive(Serialize, Deserialize, Debug)]
struct Rect {
    x: i32,
    y: i32,
    width: i32,
    height: i32
}

fn run_workspace_command(pre: &str, main_index: &str, sub_index: &str, suff: &str) -> Result<Output, io::Error> {
    Command::new("i3-msg")
        .arg(format!("workspace {}{}-{}{}", pre, main_index, sub_index, suff))
        .output()
}

fn parse_workspaces(json: &str) -> Vec<Workspace> {
    serde_json::from_str(json).unwrap()
}

fn get_matches() -> ArgMatches {
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
                    Arg::new("create")
                        .short('c')
                        .help("Create new sub workspace if one doesn't exist")
                )
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
