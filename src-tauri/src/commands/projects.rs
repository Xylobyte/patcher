use std::path::PathBuf;

use serde::Serialize;

use crate::core::configs::global_config::{ConfigState, get_config_path, RecentProject, save_config};

#[derive(Debug, Serialize)]
pub enum ProjectError {
    InvalidPath,
    InvalidProject,
}

#[tauri::command]
pub async fn get_recent_projects(state: tauri::State<'_, ConfigState>) -> Result<Vec<RecentProject>, ProjectError> {
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
pub async fn remove_project(state: tauri::State<'_, ConfigState>, app_handle: tauri::AppHandle, path: String) -> Result<Vec<RecentProject>, ProjectError> {
    let conf_dir = get_config_path(&app_handle);

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
