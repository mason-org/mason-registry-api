use http::Method;
use mason_registry_api::{
    QueryParams,
    github::{GitHubTag, api::TagResponse, client::GitHubClient, manager::GitHubManager},
    vercel::method_not_allowed,
};
use vercel_runtime::{Error, Request, Response, ResponseBody, run, service_fn};

async fn handler(request: Request) -> Result<Response<ResponseBody>, Error> {
    let api_key: String = std::env::var("GITHUB_API_KEY")?;

    if request.method() != Method::GET {
        return method_not_allowed();
    }

    let query_params: QueryParams = (&request).into();
    let tag: GitHubTag = query_params.get("tag").unwrap().parse()?;
    let repo = (&query_params).into();
    let manager = GitHubManager::new(GitHubClient::new(api_key));

    match manager.get_ref(&repo, &tag).await {
        Ok(github_ref) => mason_registry_api::vercel::ok_json::<TagResponse>(
            github_ref.into(),
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
