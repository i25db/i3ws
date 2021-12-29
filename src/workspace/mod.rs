use crate::config::Config;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct JsonWorkspace {
    pub id: i64,
    pub name: String,
    pub focused: bool,
}

pub fn parse_workspaces(json: &str) -> Vec<Workspace> {
    let json_workspaces: Vec<JsonWorkspace> =
        serde_json::from_str(json).expect("Failed to parse json string");

    let mut workspaces = Vec::<Workspace>::new();

    for json_ws in json_workspaces {
        let mut ws = Workspace::from(json_ws.name);
        ws.focused = json_ws.focused;

        workspaces.push(ws);
    }

    workspaces
}

#[derive(Clone)]
pub struct Workspace {
    pub prefix: String,
    pub main_index: String,
    pub sub_index: String,
    pub suffix: String,
    pub focused: bool,
}

impl Workspace {
    pub fn get_name(&self) -> String {
        format!(
            "{}:{}:{}:{}",
            self.prefix, self.main_index, self.sub_index, self.suffix
        )
    }
}

impl From<Config> for Workspace {
    fn from(cfg: Config) -> Self {
        Self {
            prefix: cfg.prefix,
            main_index: cfg.default_main_index,
            sub_index: cfg.default_sub_index,
            suffix: cfg.default_suffix,
            focused: false,
        }
    }
}

impl<I: Into<String>> From<I> for Workspace {
    fn from(name: I) -> Workspace {
        let name = name.into();
        let split: Vec<&str> = name.split(':').collect();

        if split.len() != 4 {
            panic!("Workspace '{}' can't be parsed", name);
        }

        Self {
            prefix: split[0].to_string(),
            main_index: split[1].to_string(),
            sub_index: split[2].to_string(),
            suffix: split[3].to_string(),
            focused: false,
        }
    }
}
