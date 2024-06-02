use std::{fs, io};
use std::fs::File;
use std::sync::Mutex;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Project {
    name: String,
    path: String,
    last_opened: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    pub recent_projects: Vec<Project>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            recent_projects: Vec::new(),
        }
    }
}

pub struct ConfigState(pub Mutex<Config>);

pub fn init_config() -> Result<Config, io::Error> {
    let config = if File::open("config.json").is_err() {
        let data = serde_json::to_string(&Config::default())
            .expect("Could not serialize default config");
        fs::write("config.json", &data)
            .expect("Could not write config.json");
        data
    } else {
        fs::read_to_string("config.json")
            .expect("Could not read config.json")
    };
    let config = serde_json::from_str::<Config>(&config).expect("Could not deserialize config.json");
    Ok(config)
}
