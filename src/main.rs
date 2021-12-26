pub mod commands;
pub mod json;
pub mod app;
use app::*;

fn main() {
    let matches = get_matches();

    match matches.subcommand() {
        Some(("go", sc_matches)) => {
            let _should_create = match sc_matches.index_of("create") {
                Some(_) => true,
                _ => false
            };

            let workspace = sc_matches.value_of("workspace").unwrap();
            let index = sc_matches.value_of("index").unwrap().to_string();

            match workspace {
                "main" => {
                    // TODO: Check if default workspace exists
                    // if not try to find a different sub workspace
                    // if it exists keep the suffix

                    commands::run_workspace_command(WorkspaceName { main_index: index, ..Default::default() });
                }
                "sub" => {
                    let workspaces = json::parse_workspaces(&commands::get_workspaces());

                    // Find a focused i3ws workspace
                    let vec = workspaces.iter()
                        .filter(|ws| ws.focused && ws.name.starts_with(PREFIX))
                        .map(|ws| {
                            let name: WorkspaceName = (&ws.name).into();
                            name
                        })
                        //.map(|ws| ws.name.trim_start_matches(PREFIX).as_bytes()[0] as char)
                        .collect::<Vec<WorkspaceName>>();

                    if vec.len() > 0 {
                        let focused_name = &vec[0];
                        println!("Found focused workspace: {:?}", &focused_name);

                        let mut target_ws_name = WorkspaceName::from(focused_name);
                        target_ws_name.sub_index = index;

                        // Find if the target workspace exists
                        let vec = workspaces.iter()
                            .filter(|ws| ws.name == format(&target_ws_name))
                            .map(|ws| {
                                let name: WorkspaceName = (&ws.name).into();
                                name
                            })
                            .collect::<Vec<WorkspaceName>>();

                        // If the target workspace doesn't exist but its in a plain main workspace
                        if vec.len() == 0 && focused_name.suffix == PLAIN_SUFFIX.to_string() {
                            println!("Target workspace doesn't exist: {:?}", target_ws_name);
                            commands::run_workspace_command(target_ws_name);
                        }
                        // If the target workspace does exist go there
                        else if vec.len() == 1 {
                            println!("Target workspace exists: {:?}", target_ws_name);
                            commands::run_workspace_command(target_ws_name);
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
