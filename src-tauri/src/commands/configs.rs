use std::fs::File;

use crate::configs::global::{Config, ConfigState, GLOBAL_CONFIG_FILE, Project, save_config};

#[tauri::command]
pub async fn get_recent_projects(state: tauri::State<'_, ConfigState>) -> Result<Vec<Project>, ()> {
    let mut config = state.0.lock().unwrap();

    *config = Config {
        recent_projects: config.recent_projects
            .iter()
            .map(|p| Project {
                path_exists: Some(!File::open(p.path.clone()).is_err()),
                ..p.clone()
            })
            .collect(),
        ..config.clone()
    };

    Ok(config.recent_projects.clone())
}

#[tauri::command]
pub async fn remove_project(state: tauri::State<'_, ConfigState>, app_handle: tauri::AppHandle, path: String) -> Result<Vec<Project>, ()> {
    let mut conf_dir = app_handle.path_resolver().app_config_dir().unwrap();
    conf_dir.push(GLOBAL_CONFIG_FILE);

    let mut config = state.0.lock().unwrap();
    config.recent_projects = config.recent_projects
        .iter()
        .filter(|p| p.path != path)
        .cloned().collect();
    save_config(&config, conf_dir.to_str().unwrap());
    Ok(config.recent_projects.clone())
}
