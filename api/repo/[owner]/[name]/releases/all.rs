use http::{Method, StatusCode};
use mason_registry_api::github::{
    client::{spec::GitHubReleaseDto, GitHubClient},
    manager::GitHubManager,
    GitHubRepo,
};
use serde::Serialize;
use std::{convert::TryInto, error::Error};

use vercel_lambda::{error::VercelError, lambda, Body, IntoResponse, Request, Response};

#[derive(Serialize)]
struct ReleasesResponse(Vec<String>);

impl From<Vec<GitHubReleaseDto>> for ReleasesResponse {
    fn from(releases: Vec<GitHubReleaseDto>) -> Self {
        ReleasesResponse(releases.into_iter().map(|r| r.tag_name).collect())
    }
}

fn handler(request: Request) -> Result<impl IntoResponse, VercelError> {
    let api_key: String =
        std::env::var("GITHUB_API_KEY").map_err(|e| VercelError::new(&format!("{}", e)))?;

    if request.method() != Method::GET {
        return Ok(Response::builder()
            .status(StatusCode::METHOD_NOT_ALLOWED)
            .body(Body::Empty)?);
    }

    let url = mason_registry_api::parse_url(&request)?;
    let repo: GitHubRepo = (&url).try_into()?;
    let manager = GitHubManager::new(GitHubClient::new(api_key));
    let releases = manager.get_all_releases(&repo)?;

    mason_registry_api::json::<ReleasesResponse>(releases.into())
}

// Start the runtime with the handler
fn main() -> Result<(), Box<dyn Error>> {
    Ok(lambda!(handler))
}
