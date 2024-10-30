use http::{Method, StatusCode};
use mason_registry_api::{
    golang::{client::GolangClient, manager::GolangManager},
    vercel::parse_url,
    QueryParams,
};
use vercel_runtime::{Body, Error, Request, Response};

pub async fn handler(request: Request) -> Result<Response<Body>, Error> {
    if request.method() != Method::GET {
        return Ok(Response::builder()
            .status(StatusCode::METHOD_NOT_ALLOWED)
            .body(Body::Empty)?);
    }

    let url = parse_url(&request)?;
    let query_params: QueryParams = (&url).into();
    let golang_pkg = (&query_params).into();
    let manager = GolangManager::new(GolangClient::new());

    match manager.get_all_versions(&golang_pkg) {
        Ok(versions) => mason_registry_api::vercel::ok_json(
            versions,
            mason_registry_api::CacheControl::PublicMedium,
        ),
        Err(err) => mason_registry_api::vercel::err_json(err),
    }
}
