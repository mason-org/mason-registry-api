use http::{Method, StatusCode};
use mason_registry_api::{
    github::{client::GitHubClient, manager::GitHubManager, GitHubTag},
    QueryParams,
};
use vercel_runtime::{run, Body, Error, Request, Response};

async fn handler(request: Request) -> Result<Response<Body>, Error> {
    let api_key: String = std::env::var("GITHUB_API_KEY")?;

    if request.method() != Method::GET {
        return Ok(Response::builder()
            .status(StatusCode::METHOD_NOT_ALLOWED)
            .body(Body::Empty)?);
    }

    let url = mason_registry_api::vercel::parse_url(&request)?;
    let query_params: QueryParams = (&url).into();
    let release: GitHubTag = query_params.get("release").unwrap().parse()?;
    let repo = (&query_params).into();
    let manager = GitHubManager::new(GitHubClient::new(api_key));

    match manager.get_release_by_tag(&repo, &release) {
        Ok(release) => mason_registry_api::vercel::ok_json(
            release,
            mason_registry_api::CacheControl::PublicMedium,
        ),
        Err(err) => mason_registry_api::vercel::err_json(err),
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}
