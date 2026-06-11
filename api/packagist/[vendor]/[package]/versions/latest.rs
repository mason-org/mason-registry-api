use http::Method;
use mason_registry_api::{
    QueryParams,
    packagist::{api::PackagistResponse, client::PackagistClient, manager::PackagistManager},
    vercel::method_not_allowed,
};
use vercel_runtime::{Error, Request, Response, ResponseBody, run, service_fn};

async fn handler(request: Request) -> Result<Response<ResponseBody>, Error> {
    if request.method() != Method::GET {
        return method_not_allowed();
    }

    let query_params: QueryParams = (&request).into();
    let packagist_package = (&query_params).into();
    let manager = PackagistManager::new(PackagistClient::new());

    match manager.get_package(&packagist_package).await {
        Ok(package) => mason_registry_api::vercel::ok_json(
            PackagistResponse::from_packagist_package_dto(packagist_package.name, package),
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
