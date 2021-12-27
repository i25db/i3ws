use std::process::{Command, Output};
pub use crate::commands;
use crate::{workspace::{WorkspaceName}, config::Config, json::*};

pub fn run_workspace_command(name: WorkspaceName) -> Output {
    Command::new("i3-msg")
        .arg(format!("workspace {}", String::from(&name)))
        .output()
        .expect("Failed to execute i3-msg command")
}

pub fn get_workspaces() -> String {
    let output = Command::new("i3-msg")
                    .args(["-t", "get_workspaces"]).output()
                    .expect("Failed to execute i3-msg command");

    String::from_utf8(output.stdout).expect("Unable to convert UTF-8 to String")
}

pub fn get_focused_workspace(config: &Config) -> Option<WorkspaceName> {
    let focused = filter_workspaces(|ws, _| ws.focused && ws.name.starts_with(config.prefix.as_str()));

    if let Some(focused) = focused {
        return Some(focused[0].clone());
    }

    None
}

pub fn filter_workspaces<F>(f: F)
    -> Option<Vec<WorkspaceName>>
    where F: Fn(&&json::Workspace, WorkspaceName) -> bool {

    let ws = parse_workspaces(&commands::get_workspaces()).iter()
        .filter(|ws| {
            let wsn: WorkspaceName = WorkspaceName::from(&ws.name);
            f(ws, wsn)
        })
        .map(|ws| {
            WorkspaceName::from(&ws.name)
        }).collect::<Vec<WorkspaceName>>();

    if ws.len() == 0 {
        return None;
    }

    Some(ws)
}
