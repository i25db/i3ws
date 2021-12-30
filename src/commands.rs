use std::process::{Command, Output};
pub use crate::commands;
use crate::app;

pub fn run_workspace_command(name: app::WorkspaceName) -> Output {
    Command::new("i3-msg")
        .arg(format!("workspace {}", format(&name)))
        .output()
        .expect("Failed to execute i3-msg command")
}

pub fn format(name: &app::WorkspaceName) -> String {
    format!("{}{}-{}:{}", name.prefix, name.main_index, name.sub_index, name.suffix)
}

pub fn get_workspaces() -> String {
    let output = Command::new("i3-msg")
                    .args(["-t", "get_workspaces"]).output()
                    .expect("Failed to execute i3-msg command");

    String::from_utf8(output.stdout).expect("Unable to convert UTF-8 to String")
}

