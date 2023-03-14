use http::{Method, StatusCode};
use mason_registry_api::{
    github::{
        client::{spec::GitHubReleaseDto, GitHubClient},
        manager::GitHubManager,
    },
    QueryParams,
};
use serde::Serialize;

use vercel_runtime::{run, Body, Error, Request, Response};

#[derive(Serialize)]
struct ReleasesResponse(Vec<String>);

impl From<Vec<GitHubReleaseDto>> for ReleasesResponse {
    fn from(releases: Vec<GitHubReleaseDto>) -> Self {
        ReleasesResponse(releases.into_iter().map(|r| r.tag_name).collect())
    }
}

async fn handler(request: Request) -> Result<Response<Body>, Error> {
    let api_key: String = std::env::var("GITHUB_API_KEY")?;

    if request.method() != Method::GET {
        return Ok(Response::builder()
            .status(StatusCode::METHOD_NOT_ALLOWED)
            .body(Body::Empty)?);
    }

    let url = mason_registry_api::vercel::parse_url(&request)?;
    let query_params: QueryParams = (&url).into();
    let repo = (&query_params).into();
    let manager = GitHubManager::new(GitHubClient::new(api_key));

    match manager.get_all_releases(&repo) {
        Ok(releases) => mason_registry_api::vercel::ok_json::<ReleasesResponse>(
            releases.into(),
            mason_registry_api::CacheControl::PublicMedium,
        ),
        Err(err) => mason_registry_api::vercel::err_json(err),
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}
