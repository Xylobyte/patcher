use std::time::SystemTime;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ProjectInfo {
    pub name: String,
    pub description: String,
    pub last_edited: String,
    pub config: ProjectConfig
}

impl Default for ProjectInfo {
    fn default() -> Self {
        ProjectInfo {
            name: String::from("New api"),
            description: String::new(),
            last_edited: <SystemTime as Into<DateTime<Utc>>>::into(SystemTime::now()).to_rfc3339(),
            config: ProjectConfig::default()
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ProjectConfig {
    pub use_folder_as_url: bool,
}

impl Default for ProjectConfig {
    fn default() -> Self {
        ProjectConfig {
            use_folder_as_url: true
        }
    }
}
