use http::{Method, StatusCode};
use mason_registry_api::{
    github::{api::TagResponse, client::GitHubClient, manager::GitHubManager, GitHubRepo},
    vercel::parse_url,
    QueryParams,
};
use vercel_runtime::{Body, Error, Request, Response};

pub async fn handler(request: Request) -> Result<Response<Body>, Error> {
    let api_key: String = std::env::var("GITHUB_API_KEY")?;

    if request.method() != Method::GET {
        return Ok(Response::builder()
            .status(StatusCode::METHOD_NOT_ALLOWED)
            .body(Body::Empty)?);
    }

    let url = parse_url(&request)?;
    let query_params: QueryParams = (&url).into();
    let repo: GitHubRepo = (&query_params).into();
    let manager = GitHubManager::new(GitHubClient::new(api_key));

    match manager.get_latest_tag(&repo) {
        Ok(latest_tag) => mason_registry_api::vercel::ok_json::<TagResponse>(
            latest_tag.into(),
            mason_registry_api::CacheControl::PublicMedium,
        ),
        Err(err) => mason_registry_api::vercel::err_json(err),
    }
}
