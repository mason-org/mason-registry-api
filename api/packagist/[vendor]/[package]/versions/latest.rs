use http::{Method, StatusCode};
use mason_registry_api::{
    packagist::{api::PackagistResponse, client::PackagistClient, manager::PackagistManager},
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
    let packagist_package = (&query_params).into();
    let manager = PackagistManager::new(PackagistClient::new());

    match manager.get_package(&packagist_package) {
        Ok(package) => mason_registry_api::vercel::ok_json(
            PackagistResponse::from_packagist_package_dto(packagist_package.name, package),
            mason_registry_api::CacheControl::PublicMedium,
        ),
        Err(err) => mason_registry_api::vercel::err_json(err),
    }
}
