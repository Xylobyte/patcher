use crate::core::configs::controller::{get_config_path, save_config};
use crate::core::configs::global_config::{ConfigState, RecentProject};
use crate::core::projects::oas3_impl::OpenApiV3SpecExt;
use crate::core::projects::project::{AppProjectSate, PROJECT_ENTRY_FILE};
use oas3::OpenApiV3Spec;
use serde::Serialize;
use std::path::PathBuf;

#[derive(Debug, Serialize)]
pub enum ProjectError {
    RecentProjectsState,
    GenerateYaml,
}

#[tauri::command]
pub async fn get_recent_projects(
    state: tauri::State<'_, ConfigState>,
) -> Result<Vec<RecentProject>, ProjectError> {
    let mut config = state
        .0
        .lock()
        .map_err(|_| ProjectError::RecentProjectsState)?;

    config.recent_projects = config
        .recent_projects
        .iter()
        .map(|p| RecentProject {
            path_exists: Some(PathBuf::from(&p.path).exists()),
            ..p.clone()
        })
        .collect();

    Ok(config.recent_projects.clone())
}

#[tauri::command]
pub async fn remove_project(
    state: tauri::State<'_, ConfigState>,
    app_handle: tauri::AppHandle,
    path: String,
) -> Result<Vec<RecentProject>, ProjectError> {
    let conf_dir = get_config_path(&app_handle);
    let mut config = state
        .0
        .lock()
        .map_err(|_| ProjectError::RecentProjectsState)?;

    config.recent_projects = config
        .recent_projects
        .iter()
        .filter(|p| p.path != path)
        .cloned()
        .collect();
    save_config(&config, conf_dir.to_str().unwrap());

    Ok(config.recent_projects.clone())
}

#[tauri::command]
pub fn open_project(app_handle: tauri::AppHandle, path: String) -> Result<(), OpenApiV3Spec> {
    let path_buf = PathBuf::from(path);
    let not_a_project = OpenApiV3Spec::new(
        app_handle.package_info().version.to_string(),
        format!("{} API", path_buf.file_name().unwrap().to_str().unwrap()),
        "0.0.1".to_string(),
        "http://127.0.0.1:3000".to_string(),
    );

    let path = path_buf.join(".patcher").join(PROJECT_ENTRY_FILE);
    if !path.exists() {
        return Err(not_a_project);
    }

    Ok(())
}

#[tauri::command]
pub async fn create_project(
    state_p: tauri::State<'_, AppProjectSate>,
    state_c: tauri::State<'_, ConfigState>,
    data: OpenApiV3Spec,
    path: String,
) -> Result<(), ProjectError> {
    let mut project_state = state_p.0.lock().map_err(|_| ProjectError::RecentProjectsState)?;
    project_state.project = Some(data.clone());

    let result = oas3::to_yaml(&data).map_err(|_| ProjectError::GenerateYaml)?;

    Ok(())
}
