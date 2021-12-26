use std::collections::HashMap;

pub struct Config {
    pub default_prefix: String,
    pub default_suffix: String,
    pub default_swap_prefix: String,
    pub default_main_index: String,
    pub default_sub_index: String,
    pub swap_on_sub: bool,
    pub swap_on_main: bool,
    pub swap_on_default_only: bool,
    pub types: Vec<WorkspaceType>,
}

pub struct WorkspaceType {
    pub name: String,
    pub default_sub_workspace: String,
    pub execute_on_move: bool,
    pub growable: bool,
    pub commands: HashMap<String, Vec<String>>,
}

impl Config {
    pub fn get_type_names(&self) -> Vec<(usize, String)> {
        self.types
            .iter()
            .enumerate()
            .map(|(i, t)| (i, t.name.to_string()))
            .collect()
    }

    pub fn get_type_by_name(&self, name: &str) -> Option<&WorkspaceType> {
        let name = name.to_string();
        let pos = self.types.iter().position(|t| t.name == name);

        if let Some(pos) = pos {
            Some(&self.types[pos])
        } else {
            None
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        let mut types = Vec::<WorkspaceType>::new();

        types.push(WorkspaceType::default());

        Self {
            default_prefix: "i3ws".to_string(),
            default_suffix: "plain".to_string(),
            default_swap_prefix: "i3wsswap".to_string(),
            default_main_index: "1".to_string(),
            default_sub_index: "1".to_string(),
            swap_on_sub: false,
            swap_on_main: true,
            swap_on_default_only: true,
            types,
        }
    }
}

impl Default for WorkspaceType {
    fn default() -> Self {
        Self {
            name: String::from("plain"),
            default_sub_workspace: String::from("1"),
            execute_on_move: false,
            growable: true,
            commands: HashMap::new(),
        }
    }
}
