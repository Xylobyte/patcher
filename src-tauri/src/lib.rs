use crate::commands::projects::{
    create_project, get_recent_projects, open_project, remove_project,
};
use crate::core::configs::controller::init_config;
use crate::core::configs::global_config::{Config, ConfigState};
use crate::core::menu;
use crate::core::projects::project::{AppProject, AppProjectSate};
use std::sync::Mutex;
use tauri::{generate_handler, Manager};

mod commands;
mod core;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let context = tauri::generate_context!();

    tauri::Builder::default()
        .setup(|app| {
            let app = app.handle();

            let config_state = if let Ok(config) = init_config(&app) {
                ConfigState(Mutex::new(config))
            } else {
                ConfigState(Mutex::new(Config::default()))
            };

            app.manage(config_state);
            app.manage(AppProjectSate(Mutex::new(AppProject::default())));
            app.set_menu(menu::build(app).expect("Cannot build the menu"))?;

            Ok(())
        })
        .plugin(tauri_plugin_window_state::Builder::new().build())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(generate_handler![
            get_recent_projects,
            remove_project,
            open_project,
            create_project
        ])
        .run(context)
        .expect("Error while running tauri application");
}
