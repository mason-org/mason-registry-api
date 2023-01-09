use http::{Method, StatusCode};
use mason_registry_api::github::{
    client::{graphql::sponsors::Sponsor, GitHubClient},
    manager::GitHubManager,
};
use serde::Serialize;

use std::error::Error;

use vercel_lambda::{error::VercelError, lambda, Body, IntoResponse, Request, Response};

#[derive(Serialize)]
pub struct SponsorsResponse {
    pub current_sponsors: Vec<String>,
}

impl From<Vec<Sponsor>> for SponsorsResponse {
    fn from(current_sponsors: Vec<Sponsor>) -> Self {
        Self {
            current_sponsors: current_sponsors.into_iter().map(|s| s.login).collect(),
        }
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

    let manager = GitHubManager::new(GitHubClient::new(api_key));
    match manager.get_all_sponsors("williamboman".to_owned()) {
        Ok(sponsors) => mason_registry_api::ok_json::<SponsorsResponse>(
            sponsors.into(),
            mason_registry_api::CacheControl::PublicMedium,
        ),
        Err(err) => mason_registry_api::err_json(err),
    }
}

// Start the runtime with the handler
fn main() -> Result<(), Box<dyn Error>> {
    Ok(lambda!(handler))
}
