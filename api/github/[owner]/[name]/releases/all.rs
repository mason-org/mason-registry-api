use http::Method;
use mason_registry_api::{
    QueryParams,
    github::{
        client::{GitHubClient, spec::GitHubReleaseDto},
        manager::GitHubManager,
    },
    vercel::method_not_allowed,
};
use serde::Serialize;

use vercel_runtime::{Error, Request, Response, ResponseBody, run, service_fn};

#[derive(Serialize)]
struct ReleasesResponse(Vec<String>);

impl From<Vec<GitHubReleaseDto>> for ReleasesResponse {
    fn from(releases: Vec<GitHubReleaseDto>) -> Self {
        ReleasesResponse(releases.into_iter().map(|r| r.tag_name).collect())
    }
}

async fn handler(request: Request) -> Result<Response<ResponseBody>, Error> {
    let api_key: String = std::env::var("GITHUB_API_KEY")?;

    if request.method() != Method::GET {
        return method_not_allowed();
    }

    let query_params: QueryParams = (&request).into();
    let repo = (&query_params).into();
    let manager = GitHubManager::new(GitHubClient::new(api_key));

    match manager.get_all_releases(&repo).await {
        Ok(releases) => mason_registry_api::vercel::ok_json::<ReleasesResponse>(
            releases.into(),
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
