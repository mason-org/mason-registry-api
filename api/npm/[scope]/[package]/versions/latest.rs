use http::Method;
use mason_registry_api::{
    QueryParams,
    npm::{client::NpmClient, manager::NpmManager},
    vercel::method_not_allowed,
};
use vercel_runtime::{Error, Request, Response, ResponseBody, run, service_fn};

async fn handler(request: Request) -> Result<Response<ResponseBody>, Error> {
    if request.method() != Method::GET {
        return method_not_allowed();
    }

    let query_params: QueryParams = (&request).into();
    let npm_package = (&query_params).into();
    let manager = NpmManager::new(NpmClient::new());

    match manager.get_package(&npm_package).await {
        Ok(package) => match manager.get_latest_package_version(&package) {
            Ok(package_version) => mason_registry_api::vercel::ok_json(
                package_version,
                mason_registry_api::CacheControl::PublicMedium,
            ),
            Err(err) => mason_registry_api::vercel::err_json(err),
        },
        Err(err) => mason_registry_api::vercel::err_json(err),
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    mason_registry_api::setup_tracing();
    let service = service_fn(handler);
    run(service).await
}
