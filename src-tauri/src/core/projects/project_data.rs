use serde::{Deserialize, Serialize};
use std::ops::Not;
use uuid::Uuid;

// Enum http methods
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum RequestMethod {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum BodyContent {
    Json(String),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ApiRequest {
    pub id: String,
    pub name: String,
    pub documentation: String,
    pub url: String,
    pub is_folder: bool,
    pub method: Option<RequestMethod>,
    pub body: Option<BodyContent>,
    pub children: Option<Vec<ApiRequest>>,
}

impl ApiRequest {
    pub fn create(name: String, is_folder: bool) -> Self {
        ApiRequest {
            id: Uuid::new_v4().to_string(),
            name,
            documentation: String::new(),
            url: String::new(),
            is_folder,
            method: is_folder.not().then_some(RequestMethod::GET),
            body: is_folder.not().then_some(BodyContent::Json(String::from("{}"))),
            children: is_folder.then_some(Vec::new()),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ProjectData {
    pub api_tree: Vec<ApiRequest>,
    pub server_addr: String,
    pub root_url: String,
}

impl Default for ProjectData {
    fn default() -> Self {
        ProjectData {
            api_tree: Vec::new(),
            server_addr: String::from("http://localhost:8000"),
            root_url: String::from("/api"),
        }
    }
}
