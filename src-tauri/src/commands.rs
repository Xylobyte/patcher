use crate::config::{Config, ConfigState};

#[tauri::command]
pub async fn get_config(state: tauri::State<'_, ConfigState>) -> Result<Config, ()> {
    Ok(state.0.lock().unwrap().clone())
}
