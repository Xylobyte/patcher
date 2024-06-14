use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ApiItem {
    pub id: String,
    pub name: String,
    pub documentation: String,
    pub url: String,
    pub is_folder: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ProjectData {
    pub api_tree: Vec<ApiItem>,
}
