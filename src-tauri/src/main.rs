// Prevents additional console window on Windows in release, pub(crate) DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;

use tauri::{AboutMetadata, generate_handler, Manager, Menu, MenuItem, Submenu};

use crate::commands::configs::{get_recent_projects, remove_project};
use crate::configs::global::{Config, ConfigState, init_config};

mod configs;
mod commands;

fn main() {
    let context = tauri::generate_context!();

    let config_state = if let Ok(config) = init_config(&context) {
        ConfigState(Mutex::new(config))
    } else {
        ConfigState(Mutex::new(Config::default()))
    };

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
            app.manage(config_state);
            Ok(())
        })
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .menu(menu)
        .invoke_handler(generate_handler![
            get_recent_projects, remove_project
        ])
        .run(context)
        .expect("error while running tauri application");
}
