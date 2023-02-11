use http::{Method, StatusCode};
use mason_registry_api::{
    github::{client::GitHubClient, manager::GitHubManager, GitHubTag},
    QueryParams,
};
use std::error::Error;

use vercel_lambda::{error::VercelError, lambda, Body, IntoResponse, Request, Response};

fn handler(request: Request) -> Result<impl IntoResponse, VercelError> {
    let api_key: String =
        std::env::var("GITHUB_API_KEY").map_err(|e| VercelError::new(&format!("{}", e)))?;

    if request.method() != Method::GET {
        return Ok(Response::builder()
            .status(StatusCode::METHOD_NOT_ALLOWED)
            .body(Body::Empty)?);
    }

    let url = mason_registry_api::parse_url(&request)?;
    let query_params: QueryParams = (&url).into();
    let release: GitHubTag = query_params.get("release").unwrap().parse()?;
    let repo = (&query_params).into();
    let manager = GitHubManager::new(GitHubClient::new(api_key));

    match manager.get_release_by_tag(&repo, &release) {
        Ok(release) => {
            mason_registry_api::ok_json(release, mason_registry_api::CacheControl::PublicMedium)
        }
        Err(err) => mason_registry_api::err_json(err),
    }
}

// Start the runtime with the handler
fn main() -> Result<(), Box<dyn Error>> {
    Ok(lambda!(handler))
}
