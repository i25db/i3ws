use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::{Config, WorkspaceType};

#[derive(Serialize, Deserialize, Clone)]
pub struct TomlConfig {
    // pub default_prefix: String,
    pub default_type: Option<String>,
    // pub default_swap_prefix: Option<String>,
    // pub default_main_index: Option<u32>,
    // pub default_sub_index: Option<u32>,
    pub swap_on_sub: Option<bool>,
    pub swap_on_main: Option<bool>,
    pub swap_on_default_only: Option<bool>,
    pub types: Option<Vec<TomlWorkspaceType>>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TomlWorkspaceType {
    pub name: Option<String>,
    pub display_name: Option<String>,
    pub sub_display_name: Option<String>,
    pub default_sub_workspace: Option<u32>,
    pub execute_on_move: Option<bool>,
    pub growable: Option<bool>,
    pub commands: Option<HashMap<String, Vec<String>>>,
}

impl From<Config> for TomlConfig {
    fn from(c: Config) -> Self {
        Self {
            default_type: Some(c.default_type),
            // default_swap_prefix: Some(c.default_swap_prefix),
            // default_main_index: Some(c.default_main_index),
            // default_sub_index: Some(c.default_sub_index),
            swap_on_sub: Some(c.swap_on_sub),
            swap_on_main: Some(c.swap_on_main),
            swap_on_default_only: Some(c.swap_on_default_only),
            types: Some(c.types.iter().map(|t| TomlWorkspaceType::from(t)).collect()),
        }
    }
}

impl From<TomlConfig> for Config {
    fn from(jc: TomlConfig) -> Self {
        let mut c = Config::default();

        if let Some(default_type) = jc.default_type {
            c.default_type = default_type;
        }
        // if let Some(default_swap_prefix) = jc.default_swap_prefix {
        //     c.default_swap_prefix = default_swap_prefix;
        // }
        // if let Some(default_main_index) = jc.default_main_index {
        //     c.default_main_index = default_main_index;
        // }
        // if let Some(default_sub_index) = jc.default_sub_index {
        //     c.default_sub_index = default_sub_index;
        // }
        if let Some(swap_on_sub) = jc.swap_on_sub {
            c.swap_on_sub = swap_on_sub;
        }
        if let Some(swap_on_main) = jc.swap_on_main {
            c.swap_on_main = swap_on_main;
        }
        if let Some(types) = jc.types {
            c.types = types.iter().map(|t| WorkspaceType::from(t)).collect();
        }

        c
    }
}

impl From<TomlWorkspaceType> for WorkspaceType {
    fn from(jws: TomlWorkspaceType) -> Self {
        let mut ws_type = WorkspaceType::default();

        if let Some(name) = jws.name {
            ws_type.name = name;
        }
        if let Some(display_name) = jws.display_name {
            ws_type.display_name = display_name;
        }
        if let Some(sub_display_name) = jws.sub_display_name {
            ws_type.sub_display_name = sub_display_name;
        }
        if let Some(default_sub_workspace) = jws.default_sub_workspace {
            ws_type.default_sub_workspace = default_sub_workspace;
        }
        if let Some(execute_on_move) = jws.execute_on_move {
            ws_type.execute_on_move = execute_on_move;
        }
        if let Some(growable) = jws.growable {
            ws_type.growable = growable;
        }
        if let Some(commands) = jws.commands {
            ws_type.commands = commands
                .iter()
                .map(|(k, v)| {
                    (
                        k.parse::<u32>().expect("Command key cannot be parsed"),
                        v.clone(),
                    )
                })
                .collect();
        }

        ws_type
    }
}

impl From<&TomlWorkspaceType> for WorkspaceType {
    fn from(jws: &TomlWorkspaceType) -> Self {
        let mut ws_type = WorkspaceType::default();

        if let Some(name) = &jws.name {
            ws_type.name = name.clone();
        }
        if let Some(display_name) = &jws.display_name {
            ws_type.display_name = display_name.clone();
        }
        if let Some(sub_display_name) = &jws.sub_display_name {
            ws_type.sub_display_name = sub_display_name.clone();
        }
        if let Some(default_sub_workspace) = jws.default_sub_workspace {
            ws_type.default_sub_workspace = default_sub_workspace;
        }
        if let Some(execute_on_move) = jws.execute_on_move {
            ws_type.execute_on_move = execute_on_move;
        }
        if let Some(growable) = jws.growable {
            ws_type.growable = growable;
        }
        if let Some(commands) = &jws.commands {
            ws_type.commands = commands
                .iter()
                .map(|(k, v)| {
                    (
                        k.parse::<u32>().expect("Command key cannot be parsed"),
                        v.clone(),
                    )
                })
                .collect();
        }

        ws_type
    }
}

impl From<WorkspaceType> for TomlWorkspaceType {
    fn from(ws_type: WorkspaceType) -> Self {
        Self {
            name: Some(ws_type.name),
            display_name: Some(ws_type.display_name),
            sub_display_name: Some(ws_type.sub_display_name),
            default_sub_workspace: Some(ws_type.default_sub_workspace),
            execute_on_move: Some(ws_type.execute_on_move),
            growable: Some(ws_type.growable),
            commands: Some(
                ws_type
                    .commands
                    .iter()
                    .map(|(k, v)| (k.to_string(), v.clone()))
                    .collect(),
            ),
        }
    }
}

impl From<&WorkspaceType> for TomlWorkspaceType {
    fn from(ws_type: &WorkspaceType) -> Self {
        Self {
            name: Some(ws_type.name.to_string()),
            display_name: Some(ws_type.display_name.to_string()),
            sub_display_name: Some(ws_type.sub_display_name.to_string()),
            default_sub_workspace: Some(ws_type.default_sub_workspace),
            execute_on_move: Some(ws_type.execute_on_move),
            growable: Some(ws_type.growable),
            commands: Some(
                ws_type
                    .commands
                    .iter()
                    .map(|(k, v)| (k.to_string(), v.clone()))
                    .collect(),
            ),
        }
    }
}
