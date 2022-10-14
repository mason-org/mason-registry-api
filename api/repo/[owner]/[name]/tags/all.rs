use http::{Method, StatusCode};
use mason_registry_api::{
    github::{
        client::{GitHubClient, GitHubPagination},
        GitHubRepo,
    },
    parse_url,
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

    let query_params = parse_url(&request)?;
    let repo: GitHubRepo = (&query_params).try_into()?;
    let client = GitHubClient::new(api_key);

    let mut tags: Vec<String> = vec![];
    let mut cursor = None;

    loop {
        let response =
            client.fetch_tags(&repo, Some(GitHubPagination::MAX_PAGE_LIMIT.into()), cursor)?;
        cursor = response.data.tags.last().map(|t| t.cursor.to_owned());

        let tags_size = response.data.tags.len();

        tags.append(
            &mut response
                .data
                .tags
                .into_iter()
                .map(|t| t.node.name)
                .collect(),
        );

        if tags_size < GitHubPagination::MAX_PAGE_LIMIT.into() {
            break;
        }
    }

    mason_registry_api::api::json(tags)
}

// Start the runtime with the handler
fn main() -> Result<(), Box<dyn Error>> {
    Ok(lambda!(handler))
}
