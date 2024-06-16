use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ApiRequest {
    pub id: String,
    pub parent_id: String,
    pub name: String,
    pub documentation: String,
    pub url: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ApiFolder {
    pub id: String,
    pub parent_id: String,
    pub name: String,
    pub documentation: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ProjectData {
    pub api_tree: Vec<ApiRequest>,
}
