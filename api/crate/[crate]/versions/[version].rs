use http::{Method, StatusCode};
use mason_registry_api::{
    crates::{api::CrateResponse, manager::CratesManager},
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
    let crate_pkg = (&query_params).into();
    let version = query_params.get("version").unwrap();
    let manager = CratesManager::new();

    match manager.get_crate_version(crate_pkg, version) {
        Ok(crate_response) => mason_registry_api::vercel::ok_json(
            CrateResponse::from_crate_response(version.to_owned(), crate_response),
            mason_registry_api::CacheControl::PublicMedium,
        ),
        Err(err) => mason_registry_api::vercel::err_json(err),
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    mason_registry_api::setup_tracing();
    run(handler).await
}
