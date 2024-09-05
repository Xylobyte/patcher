use std::sync::Mutex;

use serde::{Deserialize, Serialize};

pub const GLOBAL_CONFIG_FILE: &str = "global_config.json";

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RecentProject {
    pub name: String,
    pub path: String,
    pub last_opened: String,
    pub path_exists: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    pub recent_projects: Vec<RecentProject>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            recent_projects: Vec::new(),
        }
    }
}

pub struct ConfigState(pub Mutex<Config>);
