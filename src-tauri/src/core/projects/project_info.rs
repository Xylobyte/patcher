use std::time::SystemTime;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ProjectConfig {}

impl Default for ProjectConfig {
    fn default() -> Self {
        ProjectConfig {}
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ApiVersions {
    pub version: String,
    pub description: String,
    pub file: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ProjectInfo {
    pub name: String,
    pub description: String,
    pub last_edited: String,
    pub config: ProjectConfig,
    pub api_data: Vec<ApiVersions>,
}

impl Default for ProjectInfo {
    fn default() -> Self {
        ProjectInfo {
            name: String::from("New api"),
            description: String::new(),
            last_edited: <SystemTime as Into<DateTime<Utc>>>::into(SystemTime::now()).to_rfc3339(),
            config: ProjectConfig::default(),
            api_data: Vec::new(),
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
            api_data: Vec::new(),
        }
    }
}
