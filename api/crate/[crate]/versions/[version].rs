use http::Method;
use mason_registry_api::{
    QueryParams,
    crates::{api::CrateResponse, manager::CratesManager},
    vercel::method_not_allowed,
};
use vercel_runtime::{Error, Request, Response, ResponseBody, run, service_fn};

async fn handler(request: Request) -> Result<Response<ResponseBody>, Error> {
    if request.method() != Method::GET {
        return method_not_allowed();
    }

    let query_params: QueryParams = (&request).into();
    let crate_pkg = (&query_params).into();
    let version = query_params.get("version").unwrap();
    let manager = CratesManager::new();

    match manager.get_crate_version(crate_pkg, version).await {
        Ok(crate_response) => mason_registry_api::vercel::ok_json::<CrateResponse>(
            crate_response.into(),
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
