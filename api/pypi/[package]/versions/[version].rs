use http::{Method, StatusCode};
use mason_registry_api::{
    pypi::{client::PyPiClient, manager::PyPiManager},
    vercel::parse_url,
    QueryParams,
};

use vercel_runtime::{run, Body, Error, Request, Response};

async fn handler(request: Request) -> Result<Response<Body>, Error> {
    if request.method() != Method::GET {
        return Ok(Response::builder()
            .status(StatusCode::METHOD_NOT_ALLOWED)
            .body(Body::Empty)?);
    }

    let url = parse_url(&request)?;
    let query_params: QueryParams = (&url).into();
    let pypi_package = (&query_params).into();
    let version = query_params.get("version").unwrap();
    let manager = PyPiManager::new(PyPiClient::new());

    match manager.get_project_version(&pypi_package, version) {
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
    run(handler).await
}
