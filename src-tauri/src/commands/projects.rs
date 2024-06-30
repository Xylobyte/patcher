use std::fs;
use std::path::PathBuf;

use crate::core::configs::global_config::{ConfigState, get_config_path, RecentProject, save_config};
use crate::core::projects::project_data::ProjectData;
use crate::core::projects::project_info::ProjectInfo;

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
pub async fn open_project(_state: tauri::State<'_, ConfigState>, path: String) -> Result<(), (ProjectInfo, ProjectData)> {
    let mut path_buf = PathBuf::from(path);
    path_buf.push(".patcher");
    let info_path = path_buf.join("config.json");
    let data_path = path_buf.join("data.json");

    if !info_path.exists() || !data_path.exists() {
        return Err((ProjectInfo::default(), ProjectData::default()))
    }
    let info = fs::read_to_string(info_path)
        .map_err(|_e| (ProjectInfo::default(), ProjectData::default()))?;
    let data = fs::read_to_string(data_path)
        .map_err(|_e| (ProjectInfo::default(), ProjectData::default()))?;

    Ok(())
}
