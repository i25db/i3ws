pub struct Config {
    pub default_prefix: String,
    pub default_suffix: String,
    pub default_swap_prefix: String,
    pub default_main_index: String,
    pub default_sub_index: String,
    pub swap_on_sub: bool,
    pub swap_on_main: bool,
    pub types: Vec<WorkspaceType>,
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

        types.push(WorkspaceType {
            name: "plain".to_string(),
            growable: true,
            default_ws: "1".to_string(),
            ws_commands: vec![
                WorkspaceCommand {
                    sub_ws: "1".to_string(),
                    commands: vec![String::from("")],
                },
                WorkspaceCommand {
                    sub_ws: "2".to_string(),
                    commands: vec![String::from("")],
                },
            ],
        });

        types.push(WorkspaceType {
            name: "code".to_string(),
            growable: false,
            default_ws: "1".to_string(),
            ws_commands: vec![
                WorkspaceCommand {
                    sub_ws: "1".to_string(),
                    commands: vec![String::from("")],
                },
                WorkspaceCommand {
                    sub_ws: "2".to_string(),
                    commands: vec![String::from("")],
                },
            ],
        });

        types.push(WorkspaceType {
            name: "game".to_string(),
            growable: false,
            default_ws: "1".to_string(),
            ws_commands: vec![
                WorkspaceCommand {
                    sub_ws: "1".to_string(),
                    commands: vec![String::from("")],
                },
                WorkspaceCommand {
                    sub_ws: "2".to_string(),
                    commands: vec![String::from("")],
                },
            ],
        });

        Self {
            default_prefix: "i3ws".to_string(),
            default_suffix: "plain".to_string(),
            default_swap_prefix: "i3wsswap".to_string(),
            default_main_index: "1".to_string(),
            default_sub_index: "1".to_string(),
            swap_on_sub: false,
            swap_on_main: true,
            types,
        }
    }
}

pub struct WorkspaceType {
    pub name: String,
    pub default_ws: String,
    pub growable: bool,
    pub ws_commands: Vec<WorkspaceCommand>,
}

pub struct WorkspaceCommand {
    pub sub_ws: String,
    pub commands: Vec<String>,
}
