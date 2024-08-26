// Prevents additional console window on Windows in release, pub(crate) DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;

use tauri::{generate_handler, AboutMetadata, Manager, Menu, MenuItem, Submenu};

use crate::commands::projects::{get_recent_projects, open_project, remove_project};
use crate::core::configs::global_config::{init_config, Config, ConfigState};

mod commands;
mod core;

fn main() {
    let context = tauri::generate_context!();

    let menu = Menu::new().add_submenu(Submenu::new(
        "patcher",
        Menu::new()
            .add_native_item(MenuItem::About(
                "Patcher".parse().unwrap(),
                AboutMetadata::default(),
            ))
            .add_native_item(MenuItem::Quit),
    ));

    tauri::Builder::default()
        .setup(|app| {
            let app = app.handle();
            let config_state = if let Ok(config) = init_config(&app) {
                ConfigState(Mutex::new(config))
            } else {
                ConfigState(Mutex::new(Config::default()))
            };
            app.manage(config_state);
            Ok(())
        })
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .menu(menu)
        .invoke_handler(generate_handler![
            get_recent_projects,
            remove_project,
            open_project
        ])
        .run(context)
        .expect("Error while running tauri application");
}
