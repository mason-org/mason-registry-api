use http::{Method, StatusCode};
use mason_registry_api::{
    github::GitHubRepo,
    renovate::{client::RenovateClient, manager::RenovateManager},
};
use vercel_runtime::{run, Body, Error, Request, Response};

async fn handler(request: Request) -> Result<Response<Body>, Error> {
    let api_key: String = std::env::var("GITHUB_API_KEY")?;

    if request.method() != Method::GET {
        return Ok(Response::builder()
            .status(StatusCode::METHOD_NOT_ALLOWED)
            .body(Body::Empty)?);
    }

    let manager = RenovateManager::new(RenovateClient::new(api_key));
    let registry_repo = GitHubRepo::new("mason-org".to_owned(), "mason-registry".to_owned());
    match manager.get_badge(&registry_repo) {
        Ok(badge) => {
            mason_registry_api::vercel::ok_json(badge, mason_registry_api::CacheControl::NoStore)
        }
        Err(err) => mason_registry_api::vercel::err_json(err),
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    mason_registry_api::setup_tracing();
    run(handler).await
}
