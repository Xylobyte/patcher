// Prevents additional console window on Windows in release, pub(crate) DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;

use tauri::{generate_handler, Manager};

use crate::commands::get_config;
use crate::config::{Config, ConfigState, init_config};

mod config;
mod commands;

fn main() {
    let config_state = ConfigState(Mutex::new(Config::default()));

    tauri::Builder::default()
        .setup(|app| {
            let app_conf = app.state::<ConfigState>();
            tauri::async_runtime::spawn(async move {
                if let Ok(config) = init_config() {
                    println!("Loaded config: {:?}", config);
                    let mut s = app_conf.0.lock().unwrap();
                    *s = config;
                }
            });
            Ok(())
        })
        .manage(config_state)
        .invoke_handler(generate_handler![
            get_config
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
