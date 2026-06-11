use http::Method;
use mason_registry_api::{
    github::GitHubRepo,
    renovate::{client::RenovateClient, manager::RenovateManager},
    vercel::method_not_allowed,
};
use vercel_runtime::{Error, Request, Response, ResponseBody, run, service_fn};

async fn handler(request: Request) -> Result<Response<ResponseBody>, Error> {
    let api_key: String = std::env::var("GITHUB_API_KEY")?;

    if request.method() != Method::GET {
        return method_not_allowed();
    }

    let manager = RenovateManager::new(RenovateClient::new(api_key));
    let registry_repo = GitHubRepo::new("mason-org".to_owned(), "mason-registry".to_owned());
    match manager.get_badge(&registry_repo).await {
        Ok(badge) => {
            mason_registry_api::vercel::ok_json(badge, mason_registry_api::CacheControl::NoStore)
        }
        Err(err) => mason_registry_api::vercel::err_json(err),
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    mason_registry_api::setup_tracing();
    let service = service_fn(handler);
    run(service).await
}
