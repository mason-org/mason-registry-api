use http::Method;
use mason_registry_api::{
    QueryParams,
    pypi::{client::PyPiClient, manager::PyPiManager},
    vercel::method_not_allowed,
};

use vercel_runtime::{Error, Request, Response, ResponseBody, run, service_fn};

async fn handler(request: Request) -> Result<Response<ResponseBody>, Error> {
    if request.method() != Method::GET {
        return method_not_allowed();
    }

    let query_params: QueryParams = (&request).into();
    let pypi_package = (&query_params).into();
    let manager = PyPiManager::new(PyPiClient::new());

    match manager.get_all_package_versions(&pypi_package).await {
        Ok(versions) => mason_registry_api::vercel::ok_json(
            versions,
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
