pub(crate) use crate::config::Config;

#[derive(Debug, Clone)]
pub struct WorkspaceName {
    pub prefix: String,
    pub main_index: String,
    pub sub_index: String,
    pub suffix: String,
}

impl Default for WorkspaceName {
    fn default() -> Self {
        WorkspaceName::with(Config::default())
    }
}

impl WorkspaceName {
    pub fn with(config: Config) -> Self {
        Self {
            prefix: config.prefix,
            main_index: "1".to_string(),
            sub_index: "1".to_string(),
            suffix: config.default_suffix,
        }
    }

    pub fn format(name: &WorkspaceName) -> String {
        format!(
            "{}:{}:{}:{}",
            name.prefix, name.main_index, name.sub_index, name.suffix
        )
    }
}

impl From<&WorkspaceName> for String {
    fn from(name: &WorkspaceName) -> Self {
        WorkspaceName::format(name)
    }
}

impl From<&String> for WorkspaceName {
    fn from(name: &String) -> Self {
        let split: Vec<String> = name.split(':').map(|s| s.to_string()).collect();

        if split.len() != 4 {
            panic!("Workspace '{}' can't be parsed", name);
        }

        Self {
            prefix: split[0].to_string(),
            main_index: split[1].to_string(),
            sub_index: split[2].to_string(),
            suffix: split[3].to_string(),
        }
    }
}
