use std::collections::HashMap;
use std::fs::{self, OpenOptions};
use std::io::{Read, Write};

use std::path::Path;

mod config_file;

use config_file::*;

const CONFIG_PATH: &str = "/home/i25db/.config/i3ws/";
const CONFIG_FILE: &str = "i3ws.toml";

pub struct Config {
    pub default_prefix: String,
    pub default_type: String,
    pub default_swap_prefix: String,
    pub default_main_index: u32,
    pub default_sub_index: u32,

    pub swap_on_sub: bool,
    pub swap_on_main: bool,
    pub swap_on_default_only: bool,
    pub types: Vec<WorkspaceType>,
}

pub struct WorkspaceType {
    pub name: String,
    pub display_name: String,
    pub sub_display_name: String,
    pub default_sub_workspace: u32,
    pub execute_on_move: bool,
    pub growable: bool,
    pub commands: HashMap<u32, Vec<String>>,
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

            let toml_str = toml::to_string(&TomlConfig::from(Config::default()))
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

            match toml::from_str::<TomlConfig>(&contents) {
                Ok(config) => Config::from(config),
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

impl Default for Config {
    fn default() -> Self {
        let mut types = Vec::<WorkspaceType>::new();

        types.push(WorkspaceType::default());

        Self {
            default_prefix: "i3ws".to_string(),
            default_type: "plain".to_string(),
            default_swap_prefix: "i3wsswap".to_string(),
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
        let mut commands = HashMap::<u32, Vec<String>>::new();
        commands.insert(1, vec![String::from("kitty"), String::from("qutebrowser")]);
        commands.insert(2, vec![String::from("steam")]);

        Self {
            name: String::from("plain"),
            display_name: String::from(" {index}"),
            sub_display_name: String::from("{index}"),
            default_sub_workspace: 1,
            execute_on_move: false,
            growable: true,
            commands,
        }
    }
}
