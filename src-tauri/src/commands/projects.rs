use std::path::PathBuf;

use crate::core::configs::global_config::{ConfigState, GLOBAL_CONFIG_FILE, RecentProject, save_config};

enum ProjectError {
    InvalidPath,
    InvalidProject,
}

#[tauri::command]
pub async fn get_recent_projects(state: tauri::State<'_, ConfigState>) -> Result<Vec<RecentProject>, ()> {
    let mut config = state.0.lock().unwrap();
    config.recent_projects = config.recent_projects
        .iter()
        .map(|p| RecentProject {
            path_exists: Some(PathBuf::from(p.path.clone()).exists()),
            ..p.clone()
        })
        .collect();

    Ok(config.recent_projects.clone())
}

#[tauri::command]
pub async fn remove_project(state: tauri::State<'_, ConfigState>, app_handle: tauri::AppHandle, path: String) -> Result<Vec<RecentProject>, ()> {
    let mut conf_dir = app_handle.path_resolver().app_config_dir().unwrap();
    conf_dir.push(GLOBAL_CONFIG_FILE);

    let mut config = state.0.lock().unwrap();
    config.recent_projects = config.recent_projects
        .iter()
        .filter(|p| p.path != path)
        .cloned()
        .collect();
    save_config(&config, conf_dir.to_str().unwrap());

    Ok(config.recent_projects.clone())
}

#[tauri::command]
pub async fn open_project(state: tauri::State<'_, ConfigState>, path: String) -> Result<(), ProjectError> {
    Ok(())
}
