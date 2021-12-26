use clap::{App, AppSettings, Arg, ArgSettings, ArgMatches};
pub use crate::app;

pub const PREFIX: &str = "i3ws";
pub const PLAIN_SUFFIX: &str = ":";
pub const CODE_SUFFIX: &str = ":code";
pub const GAME_SUFFIX: &str = ":game";

#[derive(Debug)]
pub struct WorkspaceName {
    pub prefix: String,
    pub main_index: String,
    pub sub_index: String,
    pub suffix: String
}

impl Default for WorkspaceName {
    fn default() -> Self {
        Self {
            prefix: String::from(PREFIX),
            main_index: String::from("1"),
            sub_index: String::from("1"),
            suffix: String::from(PLAIN_SUFFIX)
        }
    }
}

impl From<&WorkspaceName> for WorkspaceName {
    fn from(name: &WorkspaceName) -> Self {
        Self {
            prefix: String::from(&name.prefix),
            main_index: String::from(&name.main_index),
            sub_index: String::from(&name.sub_index),
            suffix: String::from(&name.suffix)
        }
    }
}

impl From<WorkspaceName> for String {
    fn from(name: WorkspaceName) -> Self {
        format(&name)
    }
}

pub fn format(name: &WorkspaceName) -> String {
    format!("{}{}-{}{}", name.prefix, name.main_index, name.sub_index, name.suffix)
}

impl Into<WorkspaceName> for String {
    fn into(self) -> WorkspaceName {
        if !self.contains(PREFIX) || !self.contains(':') || !self.contains('-') {
            return WorkspaceName::default();
        }

        let name = self.trim_start_matches(PREFIX);
        let main_index = String::from(&name[0..1]);
        let sub_index = String::from(&name[2..3]);
        let suffix = String::from(&name[name.find(':').unwrap()..]);

        WorkspaceName {
            prefix: String::from(PREFIX),
            main_index,
            sub_index,
            suffix
        }
    }
}

impl Into<WorkspaceName> for &String {
    fn into(self) -> WorkspaceName {
        if !self.contains(PREFIX) || !self.contains(':') || !self.contains('-') {
            return WorkspaceName::default();
        }

        let name = self.trim_start_matches(PREFIX);
        let main_index = String::from(&name[0..1]);
        let sub_index = String::from(&name[2..3]);
        let suffix = String::from(&name[name.find(':').unwrap()..]);

        WorkspaceName {
            prefix: String::from(PREFIX),
            main_index,
            sub_index,
            suffix
        }
    }
}

pub fn get_matches() -> ArgMatches {
    App::new("i3ws")
        .author("i25db <i25.db@outlook.com>")
        .version("v0.0.1")
        .about("A CLI tool for managing i3 workspaces")
        .setting(AppSettings::SubcommandRequired)
        .setting(AppSettings::DisableHelpSubcommand)
        .subcommand(
            App::new("go")
                .short_flag('g')
                .about("Go to a workspace")
                .arg(
                    Arg::new("create")
                        .short('c')
                        .help("Create new sub workspace if one doesn't exist")
                )
                .arg(
                    Arg::new("workspace")
                        .takes_value(true)
                        .possible_values(["main", "sub"])
                        .setting(ArgSettings::Required)
                )
                .arg(
                    Arg::new("index")
                        .takes_value(true)
                        .possible_values(["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"])
                        .default_value("0")
                    )
                .setting(AppSettings::ArgRequiredElseHelp)
        )
        .subcommand(
            App::new("new")
                .short_flag('n')
                .about("Creates a new preset workspace")
                .arg(
                    Arg::new("new")
                        .takes_value(true)
                        .possible_values(&["plain", "code", "game"])
                        .setting(ArgSettings::Required)
                )
        )
        .get_matches()
}
