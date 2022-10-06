use github_api_proxy::{
    get_query_params,
    github::{
        client::{queries::latest_tag::LatestTagQueryResponse, GitHubClient},
        GitHubRepo,
    },
};
use http::{
    header::{CACHE_CONTROL, CONTENT_TYPE},
    Method, StatusCode,
};
use std::{convert::TryInto, error::Error};

use vercel_lambda::{error::VercelError, lambda, Body, IntoResponse, Request, Response};

fn handler(request: Request) -> Result<impl IntoResponse, VercelError> {
    let api_key: String =
        std::env::var("GITHUB_API_KEY").map_err(|e| VercelError::new(&format!("{}", e)))?;

    if request.method() != Method::GET {
        return Ok(Response::builder()
            .status(StatusCode::METHOD_NOT_ALLOWED)
            .body(Body::Empty)?);
    }

    let query_params = get_query_params(&request)?;
    let repo: GitHubRepo = query_params.try_into()?;

    let client = GitHubClient::new(api_key);

    let json: LatestTagQueryResponse = client
        .fetch_latest_tag(&repo)?
        .json()
        .map_err(|e| VercelError::new(&format!("Failed to parse JSON. {}", e)))?;

    let response = Response::builder()
        .status(200)
        .header(CONTENT_TYPE, "application/json")
        .header(CACHE_CONTROL, "public, s-maxage=1800")
        .body(Body::Text(serde_json::to_string_pretty(&json).map_err(
            |e| VercelError::new(&format!("Failed to serialize JSON. {}", e)),
        )?))?;

    Ok(response)
}

// Start the runtime with the handler
fn main() -> Result<(), Box<dyn Error>> {
    Ok(lambda!(handler))
}
