use std::fs;
use std::path::PathBuf;

use crate::core::configs::controller::{get_config_path, save_config};
use crate::core::configs::global_config::{ConfigState, RecentProject};
use crate::core::projects::project_data::ProjectData;
use crate::core::projects::project_info::ProjectInfo;
use serde::Serialize;

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
    let path_buf = PathBuf::from(path);
    let error_response = (ProjectInfo::default_with_name(format!("{} API", path_buf.file_name().unwrap().to_str().unwrap())), ProjectData::default());
    let path_buf = path_buf.join(".patcher");
    let info_path = path_buf.join("config.json");
    let data_path = path_buf.join("data.json");

    if !info_path.exists() || !data_path.exists() {
        return Err(error_response);
    }

    fn read_file<T: Default + for<'d> serde::Deserialize<'d>>(path: &PathBuf, error_r: (ProjectInfo, ProjectData)) -> Result<T, (ProjectInfo, ProjectData)> {
        fs::read_to_string(path)
            .map_err(|_| error_r.clone())
            .and_then(|content| serde_json::from_str(&content)
                .map_err(|_| error_r))
    }

    let _info: ProjectInfo = read_file(&info_path, error_response.clone())?;
    let _data: ProjectData = read_file(&data_path, error_response.clone())?;

    Ok(())
}
