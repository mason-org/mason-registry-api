use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenVSXExtensionVersionsDto {
    pub versions: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenVSXExtensionDto {
    pub namespace: String,
    pub name: String,
    pub version: String,
}
