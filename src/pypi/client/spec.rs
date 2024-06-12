use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize, Serialize)]
pub struct PyPiProjectInfoDto {
    pub name: String,
    pub version: String,
    pub license: Option<String>, // probably not an option idk
    pub requires_python: Option<String>,
}

#[derive(Deserialize)]
pub struct PyPiProjectDto {
    pub info: PyPiProjectInfoDto,
    pub releases: HashMap<String, Value>,
}

#[derive(Deserialize)]
pub struct PyPiProjectVersionedDto {
    pub info: PyPiProjectInfoDto,
}
