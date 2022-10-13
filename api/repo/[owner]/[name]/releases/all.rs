use http::{Method, StatusCode};
use mason_registry_api::github::{
    client::{GitHubClient, GitHubPagination},
    GitHubRepo,
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

    let url = mason_registry_api::parse_url(&request)?;
    let repo: GitHubRepo = (&url).try_into()?;

    let client = GitHubClient::new(api_key);

    let releases = client.paginate(
        || {
            client.fetch_releases(
                &repo,
                Some(GitHubPagination {
                    page: 1,
                    per_page: GitHubPagination::MAX_PAGE_LIMIT,
                }),
            )
        },
        |_| true,
    )?;

    mason_registry_api::api::json::<Vec<String>>(releases.into_iter().map(|r| r.tag_name).collect())
}

// Start the runtime with the handler
fn main() -> Result<(), Box<dyn Error>> {
    Ok(lambda!(handler))
}
