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
    let version = query_params.get("version").unwrap();
    let manager = PyPiManager::new(PyPiClient::new());

    match manager.get_project_version(&pypi_package, version).await {
        Ok(package) => mason_registry_api::vercel::ok_json(
            package.info,
            mason_registry_api::CacheControl::PublicLong,
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
