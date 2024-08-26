use std::time::SystemTime;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ProjectInfo {
    pub name: String,
    pub description: String,
    pub last_edited: String,
    pub config: ProjectConfig,
}

impl Default for ProjectInfo {
    fn default() -> Self {
        ProjectInfo {
            name: String::from("New api"),
            description: String::new(),
            last_edited: <SystemTime as Into<DateTime<Utc>>>::into(SystemTime::now()).to_rfc3339(),
            config: ProjectConfig::default(),
        }
    }
}

impl ProjectInfo {
    pub fn default_with_name(name: String) -> Self {
        ProjectInfo {
            name,
            description: String::new(),
            last_edited: <SystemTime as Into<DateTime<Utc>>>::into(SystemTime::now()).to_rfc3339(),
            config: ProjectConfig::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ProjectConfig {
    pub use_folders_as_url: bool,
    pub root_url: String,
}

impl Default for ProjectConfig {
    fn default() -> Self {
        ProjectConfig {
            use_folders_as_url: true,
            root_url: String::from("http://localhost:8000"),
        }
    }
}
