use crate::workspace::{self, Workspace};

use std::process::{Command, Stdio};

use crate::check_error;

pub fn activate_workspace(ws: &str) {
    check_error!(Command::new("i3-msg")
        .arg(format!("workspace {}", ws))
        .output(), "Failed to execute i3-msg command: {}");
}

pub fn move_workspace(source: &str, dest: &str, focus_dest: bool) {
    let mut arg = format!("[workspace={}] move to workspace {}", source, dest);

    if focus_dest {
        arg.push_str(&format!(", workspace {}", dest))
    }

    check_error!(Command::new("i3-msg")
        .arg(arg)
        .output(), "Failed to execute i3-msg command: {}");
}

pub fn get_workspaces() -> Vec<Workspace> {
    let output = check_error!(Command::new("i3-msg")
        .args(["-t", "get_workspaces"])
        .output(), "Failed to execute i3-msg command: {}");

    workspace::parse_workspaces(
        check_error!(&String::from_utf8(output.stdout), "Unable to convert UTF-8 to String: {}"),
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
    let i3msg = check_error!(Command::new("i3-msg")
        .args(["-t", "get_tree"])
        .stdout(Stdio::piped())
        .spawn(), "Failed to execute i3-msg command: {}");

    let jq = check_error!(Command::new("jq")
        .arg(jq_arg)
        .stdin(i3msg.stdout.unwrap())
        .output(), "Failed to execute jq: {}");

    String::from_utf8_lossy(&jq.stdout) == String::from("")
}
