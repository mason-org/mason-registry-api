use http::{Method, StatusCode};
use mason_registry_api::github::{client::GitHubClient, GitHubRepo};
use std::{collections::HashMap, convert::TryInto, error::Error};

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
    let repo: GitHubRepo = (&url).try_into()?;

    let client = GitHubClient::new(api_key);

    mason_registry_api::api::json(client.fetch_latest_release(
        &repo,
        mason_registry_api::url_has_query_flag(&url, "include_prerelease"),
    )?)
}

// Start the runtime with the handler
fn main() -> Result<(), Box<dyn Error>> {
    Ok(lambda!(handler))
}
