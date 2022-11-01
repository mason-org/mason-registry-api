use http::{Method, StatusCode};
use mason_registry_api::{
    parse_url,
    rubygems::{api::RubyGemResponse, client::RubyGemsClient, manager::RubyGemsManager},
    QueryParams,
};

use std::error::Error;

use vercel_lambda::{error::VercelError, lambda, Body, IntoResponse, Request, Response};

fn handler(request: Request) -> Result<impl IntoResponse, VercelError> {
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
        Ok(versioned_gem) => mason_registry_api::ok_json(RubyGemResponse::from_versioned_dto(
            gem.name,
            versioned_gem,
        )),
        Err(err) => mason_registry_api::err_json(err),
    }
}

// Start the runtime with the handler
fn main() -> Result<(), Box<dyn Error>> {
    Ok(lambda!(handler))
}
