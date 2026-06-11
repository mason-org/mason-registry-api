use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenVSXExtensionVersionsDto {
    pub versions: HashMap<String, String>,
    pub total_size: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenVSXExtensionDto {
    pub namespace: String,
    pub name: String,
    pub version: String,
}
