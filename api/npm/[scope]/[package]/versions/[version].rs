use http::{Method, StatusCode};
use mason_registry_api::{
    npm::{client::NpmClient, manager::NpmManager},
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
    let npm_package = (&query_params).into();
    let version = query_params.get("version").unwrap();
    let manager = NpmManager::new(NpmClient::new());

    match manager.get_package(&npm_package) {
        Ok(package) => match manager.get_package_version(&package, version) {
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
    run(handler).await
}
