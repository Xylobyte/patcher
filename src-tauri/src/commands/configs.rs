use std::fs::File;

use crate::configs::global::{Config, ConfigState, Project};

#[tauri::command]
pub async fn get_recent_projects(state: tauri::State<'_, ConfigState>) -> Result<Vec<Project>, ()> {
    let mut config = state.0.lock().unwrap();

    *config = Config {
        recent_projects: config.recent_projects.iter().map(|p| {
            Project {
                path_exists: Some(!File::open(p.path.clone()).is_err()),
                ..p.clone()
            }
        }).collect(),
        ..config.clone()
    };

    Ok(config.clone().recent_projects)
}
