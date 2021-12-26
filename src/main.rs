pub mod commands;
pub mod json;
pub mod app;
use app::*;

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
                    commands::run_workspace_command(app::WorkspaceName { main_index: index.to_string(), ..Default::default() });
                }
                "sub" => {
                    let workspaces = json::parse_workspaces(&commands::get_workspaces());

                    // Find a focused i3ws workspace, trim the prefix off and get the first
                    // character
                    let vec = workspaces.iter()
                        .filter(|ws| ws.focused && ws.name.starts_with(PREFIX))
                        .map(|ws| ws.name.trim_start_matches(PREFIX).as_bytes()[0] as char)
                        .collect::<Vec<char>>();

                    if vec.len() > 0 {
                        let main_index = vec[0];

                        let ws_name = app::WorkspaceName { main_index: String::from(main_index), sub_index: index.to_string(), ..Default::default()};
                        if should_create {
                            commands::run_workspace_command(ws_name);
                        }
                        else if workspaces.iter()
                            .filter(|ws| ws.name == commands::format(&ws_name))
                            .collect::<Vec<_>>()
                            .len() == 1 {
                            commands::run_workspace_command(ws_name);
                        }
                    }

                    /*for ws in &workspaces {
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
                    }*/
                    // If there was an existing main workspace
                    /*if let Some(main_index) = main_index {
                        // switch to/create sub workspace
                        if should_create {
                            commands::run_workspace_command(ws_name);
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
                    }*/
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
