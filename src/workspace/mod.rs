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

#[derive(Clone, Debug)]
pub struct Workspace {
    pub prefix: String,
    pub main_index: u32,
    pub sub_index: u32,
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
            prefix: cfg.default_prefix,
            main_index: cfg.default_main_index,
            sub_index: cfg.default_sub_index,
            suffix: cfg.default_type,
            focused: false,
        }
    }
}

impl<I: Into<String>> From<I> for Workspace {
    fn from(name: I) -> Workspace {
        let name = name.into();
        let split: Vec<&str> = name.split(':').collect();

        if split.len() != 4 {
            return Workspace::from(Config::default());
        }

        let main_index = split[1]
            .parse::<u32>()
            .unwrap_or(Config::default().default_main_index);
        let sub_index = split[2]
            .parse::<u32>()
            .unwrap_or(Config::default().default_sub_index);

        Self {
            prefix: split[0].to_string(),
            main_index,
            sub_index,
            suffix: split[3].to_string(),
            focused: false,
        }
    }
}
