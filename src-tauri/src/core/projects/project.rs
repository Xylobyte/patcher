use oas3::OpenApiV3Spec;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AppProject {
    pub project: Option<OpenApiV3Spec>,
}

impl Default for AppProject {
    fn default() -> Self {
        Self { project: None }
    }
}

pub struct AppProjectSate(pub Mutex<AppProject>);
