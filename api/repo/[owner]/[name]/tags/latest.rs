use http::{Method, StatusCode};
use mason_registry_api::{
    github::{
        client::{
            graphql::{tags::TagNode, Edge},
            GitHubClient,
        },
        GitHubRepo,
    },
    parse_url,
};
use serde::Serialize;
use std::{convert::TryInto, error::Error};

use vercel_lambda::{error::VercelError, lambda, Body, IntoResponse, Request, Response};

#[derive(Serialize)]
struct TagResponse {
    pub tag: String,
}

impl From<Edge<TagNode>> for TagResponse {
    fn from(edge: Edge<TagNode>) -> Self {
        Self {
            tag: edge.node.name,
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

    let query_params = parse_url(&request)?;
    let repo: GitHubRepo = (&query_params).try_into()?;
    let client = GitHubClient::new(api_key);

    mason_registry_api::api::json::<TagResponse>(client.fetch_latest_tag(&repo)?.data.into())
}

// Start the runtime with the handler
fn main() -> Result<(), Box<dyn Error>> {
    Ok(lambda!(handler))
}
