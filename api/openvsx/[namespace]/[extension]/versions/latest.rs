use http::{Method, StatusCode};
use mason_registry_api::{
    openvsx::{api::OpenVSXExtensionResponse, client::OpenVSXClient, manager::OpenVSXManager},
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
    let extension = (&query_params).into();
    let manager = OpenVSXManager::new(OpenVSXClient::new());

    match manager.get_extension(&extension) {
        Ok(extension_dto) => mason_registry_api::vercel::ok_json::<OpenVSXExtensionResponse>(
            extension_dto.into(),
            mason_registry_api::CacheControl::PublicMedium,
        ),
        Err(err) => mason_registry_api::vercel::err_json(err),
    }
}
