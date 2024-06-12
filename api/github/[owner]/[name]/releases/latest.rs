use http::{Method, StatusCode};
use mason_registry_api::{
    github::{client::GitHubClient, manager::GitHubManager},
    CacheControl, QueryParams,
};
use vercel_runtime::{Body, Error, Request, Response};

pub async fn handler(request: Request) -> Result<Response<Body>, Error> {
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

    match manager.get_latest_release(&repo) {
        Ok(latest_release) => {
            mason_registry_api::vercel::ok_json(latest_release, CacheControl::PublicShort)
        }
        Err(err) => mason_registry_api::vercel::err_json(err),
    }
}
