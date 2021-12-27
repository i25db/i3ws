use serde::Serialize;
use serde::Deserialize;

pub use crate::config;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub prefix: String,
    pub default_suffix: String,
    pub types: Vec<WorkspaceType>
}

impl Config {
    pub fn get_type_names(&self) -> Vec<(usize, String)> {
        self.types.iter().enumerate()
            .map(|(i, t)| (i, t.name.to_string()))
            .collect()
    }

    pub fn get_type_by_name(&self, name: String) -> Option<&WorkspaceType> {
        let pos = self.types.iter().position(|t| t.name == name);

        if let Some(pos) = pos {
            Some(&self.types[pos])
        }
        else {
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
                WorkspaceCommand { sub_ws: "1".to_string(), commands: vec![String::from(""),]},
                WorkspaceCommand { sub_ws: "2".to_string(), commands: vec![String::from(""),]},
            ]
        });

        types.push(WorkspaceType {
            name: "code".to_string(),
            growable: false,
            default_ws: "1".to_string(),
            ws_commands: vec![
                WorkspaceCommand { sub_ws: "1".to_string(), commands: vec![String::from(""),]},
                WorkspaceCommand { sub_ws: "2".to_string(), commands: vec![String::from(""),]},
            ]
        });

        types.push(WorkspaceType {
            name: "game".to_string(),
            growable: false,
            default_ws: "1".to_string(),
            ws_commands: vec![
                WorkspaceCommand { sub_ws: "1".to_string(), commands: vec![String::from(""),]},
                WorkspaceCommand { sub_ws: "2".to_string(), commands: vec![String::from(""),]},
            ]
        });

        Self {
            prefix: "i3ws".to_string(),
            default_suffix: "plain".to_string(),
            types
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct WorkspaceType {
    pub name: String,
    pub default_ws: String,
    pub growable: bool,
    pub ws_commands: Vec<WorkspaceCommand>
}

#[derive(Serialize, Deserialize)]
pub struct WorkspaceCommand {
    pub sub_ws: String,
    pub commands: Vec<String>
}
