use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ApiRequest {
    pub id: String,
    pub name: String,
    pub documentation: String,
    pub url: String,
    pub is_folder: bool,
    pub children: Vec<ApiRequest>,
}

impl Default for ApiRequest {
    fn default() -> Self {
        ApiRequest {
            id: Uuid::new_v4().to_string(),
            name: String::from("New request"),
            documentation: String::new(),
            url: String::new(),
            is_folder: false,
            children: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ProjectData {
    pub api_tree: Vec<ApiRequest>,
}

impl Default for ProjectData {
    fn default() -> Self {
        ProjectData {
            api_tree: Vec::new(),
        }
    }
}
