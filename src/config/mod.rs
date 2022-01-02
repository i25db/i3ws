use std::collections::HashMap;
use std::fs::{self, OpenOptions};
use std::io::{Read, Write};

use std::path::Path;
use serde::{Serialize, Deserialize};

mod default;

const CONFIG_PATH: &str = "/home/i25db/.config/i3ws/";
const CONFIG_FILE: &str = "i3ws.toml";

#[derive(Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "Config::default_prefix", skip)]
    pub default_prefix: String,


    #[serde(default = "Config::default_swap_prefix", skip)]
    pub default_swap_prefix: String,
    pub default_type: String,
    
    #[serde(default = "Config::default_main_index")]
    pub default_main_index: u32,

    #[serde(default = "Config::default_sub_index")]
    pub default_sub_index: u32,


    #[serde(default = "Config::default_swap_on_sub")]
    pub swap_on_sub: bool,

    #[serde(default = "Config::default_swap_on_main")]
    pub swap_on_main: bool,

    #[serde(default = "Config::default_swap_on_default_only")]
    pub swap_on_default_only: bool,

    #[serde(default = "Config::default_types")]
    pub types: Vec<WorkspaceType>,
}

#[derive(Serialize, Deserialize)]
pub struct WorkspaceType {
    pub name: String,


    #[serde(default = "WorkspaceType::default_display_name")]
    pub display_name: String,

    #[serde(default = "WorkspaceType::default_display_name_focused")]
    pub display_name_focused: String,

    #[serde(default = "WorkspaceType::default_sub_display_name")]
    pub sub_display_name: String,

    #[serde(default = "WorkspaceType::default_sub_display_name_focused")]
    pub sub_display_name_focused: String,

    #[serde(default = "WorkspaceType::default_display_sep")]
    pub display_sep: String,


    #[serde(default = "WorkspaceType::default_max_sub_count")]
    pub max_sub_count: u32,


    #[serde(default = "WorkspaceType::default_sub_workspace")]
    pub default_sub_workspace: u32,

    #[serde(default = "WorkspaceType::default_execute_on_move")]
    pub execute_on_move: bool,

    #[serde(default = "WorkspaceType::default_growable")]
    pub growable: bool,

    #[serde(default = "WorkspaceType::default_commands")]
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

    pub fn get_config_from_file() -> Config {
        let path = Path::new(CONFIG_PATH);

        if !path.exists() {
            if let Err(err) = fs::create_dir_all(path) {
                panic!("Error creating config directory: {}", err);
            }
        }

        let path_str = format!("{}{}", CONFIG_PATH, CONFIG_FILE);
        let path = Path::new(&path_str);

        if !path.exists() {
            let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .open(path_str)
                .expect("Error opening config file.");

            let toml_str = toml::to_string(&Config::default())
                .expect("Error parsing config object to toml");

            file.write_all(toml_str.as_bytes())
                .expect("Error writing config file");

            Config::default()
        } else {
            let mut file = OpenOptions::new()
                .read(true)
                .open(path_str)
                .expect("Error opening config file.");

            let mut contents = String::new();
            file.read_to_string(&mut contents)
                .expect("Error reading config file");

            match toml::from_str::<Config>(&contents) {
                Ok(config) => config,
                Err(err) => {
                    println!("Error parsing config file: {}.\n Using default config", err);
                    Config::default()
                }
            }
        }
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
