use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct RubyGemDto {
    pub name: String,
    pub info: String,
    pub version: String,
    pub licenses: Option<Vec<String>>,
}

#[derive(Deserialize, Serialize)]
pub struct RubyGemVersionDto {
    pub description: String,
    #[serde(rename(deserialize = "number"))]
    pub version: String,
    pub licenses: Option<Vec<String>>,
    pub prerelease: bool,
}

#[derive(Deserialize)]
pub struct RubyLatestVersionDto {
    pub version: String,
}
