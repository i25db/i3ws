use crate::workspace::{self, Workspace};

use std::process::{Command, Output, Stdio};

pub fn run_workspace_command(ws: Workspace) -> Output {
    Command::new("i3-msg")
        .arg(format!("workspace {}", ws.get_name()))
        .output()
        .expect("Failed to execute i3-msg command")
}

pub fn get_workspaces() -> Vec<Workspace> {
    let output = Command::new("i3-msg")
        .args(["-t", "get_workspaces"])
        .output()
        .expect("Failed to execute i3-msg command");

    workspace::parse_workspaces(
        &String::from_utf8(output.stdout).expect("Unable to convert UTF-8 to String"),
    )
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
