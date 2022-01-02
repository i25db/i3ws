use super::{Config, WorkspaceType};
use std::collections::HashMap;

impl Config {
    pub fn default_prefix() -> String {
        Config::default().default_prefix
    }

    pub fn default_swap_prefix() -> String {
        Config::default().default_swap_prefix
    }

    pub fn default_main_index() -> u32 {
        Config::default().default_main_index
    }

    pub fn default_sub_index() -> u32 {
        Config::default().default_sub_index
    }

    pub fn default_swap_on_main() -> bool {
        Config::default().swap_on_main
    }

    pub fn default_swap_on_sub() -> bool {
        Config::default().swap_on_sub
    }

    pub fn default_swap_on_default_only() -> bool {
        Config::default().swap_on_default_only
    }

    pub fn default_types() -> Vec<WorkspaceType> {
        Config::default().types
    }
}

impl WorkspaceType {
    pub fn default_display_name() -> String {
        WorkspaceType::default().display_name
    }

    pub fn default_display_name_focused() -> String {
        WorkspaceType::default().display_name_focused
    }

    pub fn default_sub_display_name() -> String {
        WorkspaceType::default().sub_display_name
    }

    pub fn default_sub_display_name_focused() -> String {
        WorkspaceType::default().sub_display_name_focused
    }

    pub fn default_display_sep() -> String {
        WorkspaceType::default().display_sep
    }

    pub fn default_display_name_empty() -> String {
        WorkspaceType::default().display_name_empty
    }

    pub fn default_sub_workspace() -> u32 {
        WorkspaceType::default().default_sub_workspace
    }

    pub fn default_execute_on_move() -> bool {
        WorkspaceType::default().execute_on_move
    }

    pub fn default_growable() -> bool {
        WorkspaceType::default().growable
    }

    pub fn default_commands() -> HashMap<String, Vec<String>> {
        WorkspaceType::default().commands
    }
}

impl Default for Config {
    fn default() -> Self {
        let mut types = Vec::<WorkspaceType>::new();

        types.push(WorkspaceType::default());

        Self {
            default_prefix: String::from("i3ws"),
            default_type: String::from("plain"),
            default_swap_prefix: String::from("i3wsswap"),
            default_main_index: 1,
            default_sub_index: 1,
            swap_on_sub: false,
            swap_on_main: true,
            swap_on_default_only: true,
            types,
        }
    }
}

impl Default for WorkspaceType {
    fn default() -> Self {
        let mut commands = HashMap::<String, Vec<String>>::new();
        commands.insert(String::from("1"), vec![String::from("kitty"), String::from("qutebrowser")]);
        commands.insert(String::from("2"), vec![String::from("steam")]);

        let display_name = String::from(" {index}");
        let sub_display_name = String::from("{index}");

        Self {
            name: String::from("plain"),

            display_name: display_name.clone(),
            display_name_focused: display_name,
            sub_display_name: sub_display_name.clone(),
            sub_display_name_focused: sub_display_name,
            display_sep: String::from(" | "),

            max_sub_count: Some(10),
            display_name_empty: String::default(),

            default_sub_workspace: 1,
            execute_on_move: false,
            growable: true,
            commands,
        }
    }
}
