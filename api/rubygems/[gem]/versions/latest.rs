use http::Method;
use mason_registry_api::{
    QueryParams,
    rubygems::{api::RubyGemResponse, client::RubyGemsClient, manager::RubyGemsManager},
    vercel::method_not_allowed,
};
use vercel_runtime::{Error, Request, Response, ResponseBody, run, service_fn};

async fn handler(request: Request) -> Result<Response<ResponseBody>, Error> {
    if request.method() != Method::GET {
        return method_not_allowed();
    }

    let query_params: QueryParams = (&request).into();
    let gem = (&query_params).into();
    let manager = RubyGemsManager::new(RubyGemsClient::new());

    match manager.get_gem(&gem).await {
        Ok(gem) => mason_registry_api::vercel::ok_json::<RubyGemResponse>(
            gem.into(),
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
