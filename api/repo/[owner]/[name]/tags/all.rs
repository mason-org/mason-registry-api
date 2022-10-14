use http::{Method, StatusCode};
use mason_registry_api::{
    github::{
        client::{
            graphql::{tags::TagNode, Edge},
            GitHubClient,
        },
        manager::GitHubManager,
    },
    parse_url,
};
use serde::Serialize;
use std::{convert::TryInto, error::Error};

use vercel_lambda::{error::VercelError, lambda, Body, IntoResponse, Request, Response};

#[derive(Serialize)]
struct TagsResponse(Vec<String>);

impl From<Vec<Edge<TagNode>>> for TagsResponse {
    fn from(edges: Vec<Edge<TagNode>>) -> Self {
        Self(edges.into_iter().map(|t| t.node.name).collect())
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

    let url = parse_url(&request)?;
    let repo = (&url).try_into()?;
    let manager = GitHubManager::new(GitHubClient::new(api_key));
    let tags = manager.get_all_tags(&repo)?;

    mason_registry_api::json::<TagsResponse>(tags.into())
}

// Start the runtime with the handler
fn main() -> Result<(), Box<dyn Error>> {
    Ok(lambda!(handler))
}
