use http::Method;
use mason_registry_api::{
    github::{
        client::{GitHubClient, graphql::sponsors::Sponsor},
        manager::GitHubManager,
    },
    vercel::method_not_allowed,
};
use serde::Serialize;
use vercel_runtime::{Error, Request, Response, ResponseBody, run, service_fn};

#[derive(Serialize)]
pub struct SponsorsResponse {
    pub current_sponsors: Vec<String>,
}

impl From<Vec<Sponsor>> for SponsorsResponse {
    fn from(current_sponsors: Vec<Sponsor>) -> Self {
        Self {
            current_sponsors: current_sponsors.into_iter().map(|s| s.login).collect(),
        }
    }
}

async fn handler(request: Request) -> Result<Response<ResponseBody>, Error> {
    let api_key: String = std::env::var("GITHUB_API_KEY")?;

    if request.method() != Method::GET {
        return method_not_allowed();
    }

    let manager = GitHubManager::new(GitHubClient::new(api_key));
    match manager.get_all_sponsors("williamboman".to_owned()).await {
        Ok(sponsors) => mason_registry_api::vercel::ok_json::<SponsorsResponse>(
            sponsors.into(),
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
