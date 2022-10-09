use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GitHubReleaseAssetDto {
    pub id: u64,
    pub url: String,
    pub name: String,
    pub browser_download_url: String,
    pub created_at: String,
    pub updated_at: String,
    pub size: u64,
    pub download_count: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GitHubReleaseDto {
    pub id: u64,
    pub tag_name: String,
    pub draft: bool,
    pub prerelease: bool,
    pub assets: Vec<GitHubReleaseAssetDto>,
}
