use crate::configs::global::{ConfigState, Project};

#[tauri::command]
pub async fn get_recent_projects(state: tauri::State<'_, ConfigState>) -> Result<Vec<Project>, ()> {
    Ok(state.0.lock().unwrap().clone().recent_projects)
}
