use std::path::PathBuf;
use std::sync::Mutex;
use std::{fs, io};

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

pub fn get_config_path(app_handle: &tauri::AppHandle) -> PathBuf {
    let mut path = app_handle
        .path_resolver()
        .app_config_dir()
        .expect("Could not get app config dir");
    path.push(GLOBAL_CONFIG_FILE);
    path
}

pub fn init_config(app_handle: &tauri::AppHandle) -> Result<Config, io::Error> {
    let config_path = get_config_path(app_handle);

    let config = if !config_path.exists() {
        let data =
            serde_json::to_string(&Config::default()).expect("Could not serialize default config");
        fs::write(config_path, &data).expect("Could not write config.json");
        data
    } else {
        fs::read_to_string(config_path).expect("Could not read config.json")
    };
    let config =
        serde_json::from_str::<Config>(&config).expect("Could not deserialize config.json");
    Ok(config)
}

pub fn save_config(config: &Config, config_path: &str) {
    let config = serde_json::to_string(config).expect("Could not stringify config");
    fs::write(config_path, &config).expect("Could not write config.json");
}
