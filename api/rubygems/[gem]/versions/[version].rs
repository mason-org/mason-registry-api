use http::{Method, StatusCode};
use mason_registry_api::{
    rubygems::{api::RubyGemResponse, client::RubyGemsClient, manager::RubyGemsManager},
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
    let gem = (&query_params).into();
    let version = query_params.get("version").unwrap();
    let manager = RubyGemsManager::new(RubyGemsClient::new());

    match manager.get_gem_version(&gem, version) {
        Ok(versioned_gem) => mason_registry_api::vercel::ok_json(
            RubyGemResponse::from_versioned_dto(gem.name, versioned_gem),
            mason_registry_api::CacheControl::PublicMedium,
        ),
        Err(err) => mason_registry_api::vercel::err_json(err),
    }
}
