use std::fs;
use std::path::PathBuf;

use serde::Serialize;

use crate::core::configs::global_config::{ConfigState, get_config_path, RecentProject, save_config};
use crate::core::projects::project_data::ProjectData;
use crate::core::projects::project_info::ProjectInfo;

#[derive(Debug, Serialize)]
pub enum ProjectError {
    RecentProjectsState,
}

#[tauri::command]
pub async fn get_recent_projects(state: tauri::State<'_, ConfigState>) -> Result<Vec<RecentProject>, ProjectError> {
    let mut config = state.0.lock()
        .map_err(|_| ProjectError::RecentProjectsState)?;

    config.recent_projects = config.recent_projects
        .iter()
        .map(|p| RecentProject {
            path_exists: Some(PathBuf::from(&p.path).exists()),
            ..p.clone()
        })
        .collect();

    Ok(config.recent_projects.clone())
}

#[tauri::command]
pub async fn remove_project(state: tauri::State<'_, ConfigState>, app_handle: tauri::AppHandle, path: String) -> Result<Vec<RecentProject>, ProjectError> {
    let conf_dir = get_config_path(&app_handle);
    let mut config = state.0.lock()
        .map_err(|_| ProjectError::RecentProjectsState)?;

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
    let mut path_buf = PathBuf::from(path).join(".patcher");
    let info_path = path_buf.join("config.json");
    let data_path = path_buf.join("data.json");

    fn read_file<T: Default + for<'d> serde::Deserialize<'d>>(path: &PathBuf) -> Result<T, (ProjectInfo, ProjectData)> {
        fs::read_to_string(path)
            .map_err(|_| (ProjectInfo::default(), ProjectData::default()))
            .and_then(|content| serde_json::from_str(&content).map_err(|_| (ProjectInfo::default(), ProjectData::default())))
    }

    if !info_path.exists() || !data_path.exists() {
        return Err((ProjectInfo::default(), ProjectData::default()));
    }

    let info: ProjectInfo = read_file(&info_path)?;
    let data: ProjectData = read_file(&data_path)?;

    Ok(())
}
