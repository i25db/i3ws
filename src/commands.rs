pub(crate) use crate::commands;
use crate::{config::Config, json::*, workspace::WorkspaceName};
pub(crate) use std::process::{Command, Output, Stdio};

pub fn run_workspace_command(name: &WorkspaceName) -> Output {
    Command::new("i3-msg")
        .arg(format!("workspace {}", String::from(name)))
        .output()
        .expect("Failed to execute i3-msg command")
}

pub fn get_workspaces() -> String {
    let output = Command::new("i3-msg")
        .args(["-t", "get_workspaces"])
        .output()
        .expect("Failed to execute i3-msg command");

    String::from_utf8(output.stdout).expect("Unable to convert UTF-8 to String")
}

pub fn get_focused_workspace(config: &Config) -> Option<WorkspaceName> {
    let focused =
        filter_workspaces(|ws, _| ws.focused && ws.name.starts_with(config.prefix.as_str()));

    if let Some(focused) = focused {
        return Some(focused[0].clone());
    }

    None
}

pub fn filter_workspaces<F>(f: F) -> Option<Vec<WorkspaceName>>
where
    F: Fn(&&json::Workspace, WorkspaceName) -> bool,
{
    let ws = parse_workspaces(&commands::get_workspaces())
        .iter()
        .filter(|ws| {
            let wsn: WorkspaceName = WorkspaceName::from(&ws.name);
            f(ws, wsn)
        })
        .map(|ws| WorkspaceName::from(&ws.name))
        .collect::<Vec<WorkspaceName>>();

    if ws.len() == 0 {
        return None;
    }

    Some(ws)
}

pub fn is_focused_workspace_empty(config: &Config) -> bool {
    if let Some(focused) = get_focused_workspace(config) {
        if is_workspace_empty(WorkspaceName::format(&focused)) {
            // if the focused workspace is empty
            if let Some(sub_ws) = filter_workspaces(|_, wsn| wsn.main_index == focused.main_index) {
                // and it is the only workspace with it's main_index
                if sub_ws.len() == 1 {
                    return true;
                }
            }
        }
    }

    false
}

pub fn is_workspace_empty(ws: String) -> bool {
    let jq_arg = format!(
        ".nodes[] \
            | select(.name != \"__i3\") \
            | .nodes[] \
            | .nodes[] \
            | select(.name == \"{}\") \
            | .nodes[] \
            | if .window != null then \"true\" else \"\" end",
        ws
    );
    let i3msg = Command::new("i3-msg")
        .args(["-t", "get_tree"])
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to execute i3-msg command");

    let jq = Command::new("jq")
        .arg(jq_arg)
        .stdin(i3msg.stdout.unwrap())
        .output()
        .expect("Failed to execute jq");

    String::from_utf8_lossy(&jq.stdout) == String::from("")
}
