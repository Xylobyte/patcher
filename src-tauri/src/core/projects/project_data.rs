use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ApiRequest {
    pub id: String,
    pub name: String,
    pub documentation: String,
    pub url: String,
    pub is_folder: bool,
    pub children: Vec<ApiRequest>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ProjectData {
    pub api_tree: Vec<ApiRequest>,
    pub root_url: String,
}
