use http::Method;
use mason_registry_api::{
    QueryParams,
    github::{GitHubTag, client::GitHubClient, manager::GitHubManager},
    vercel::method_not_allowed,
};
use vercel_runtime::{Error, Request, Response, ResponseBody, run, service_fn};

async fn handler(request: Request) -> Result<Response<ResponseBody>, Error> {
    let api_key: String = std::env::var("GITHUB_API_KEY")?;

    if request.method() != Method::GET {
        return method_not_allowed();
    }

    let query_params: QueryParams = (&request).into();
    let release: GitHubTag = query_params.get("release").unwrap().parse()?;
    let repo = (&query_params).into();
    let manager = GitHubManager::new(GitHubClient::new(api_key));

    match manager.get_release_by_tag(&repo, &release).await {
        Ok(release) => mason_registry_api::vercel::ok_json(
            release,
            mason_registry_api::CacheControl::PublicMedium,
        ),
        Err(err) => mason_registry_api::vercel::err_json(err),
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    mason_registry_api::setup_tracing();
    let service = service_fn(handler);
    run(service).await
}
