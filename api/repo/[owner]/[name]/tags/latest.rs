use http::{Method, StatusCode};
use mason_registry_api::{
    github::{
        client::{
            graphql::{tags::TagNode, Edge},
            GitHubClient,
        },
        manager::GitHubManager,
        GitHubRepo,
    },
    parse_url, QueryParams,
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

    let url = parse_url(&request)?;
    let query_params: QueryParams = (&url).into();
    let repo: GitHubRepo = (&query_params).try_into()?;
    let manager = GitHubManager::new(GitHubClient::new(api_key));

    match manager.get_latest_tag(&repo) {
        Ok(latest_tag) => mason_registry_api::ok_json::<TagResponse>(latest_tag.into()),
        Err(err) => mason_registry_api::err_json(err),
    }
}

// Start the runtime with the handler
fn main() -> Result<(), Box<dyn Error>> {
    Ok(lambda!(handler))
}
