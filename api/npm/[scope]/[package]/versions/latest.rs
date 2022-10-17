use http::{Method, StatusCode};
use mason_registry_api::{
    npm::{client::NpmClient, manager::NpmManager},
    parse_url, QueryParams,
};
use serde::Serialize;
use std::{convert::TryInto, error::Error};

use vercel_lambda::{error::VercelError, lambda, Body, IntoResponse, Request, Response};

#[derive(Serialize)]
struct LatestVersionResponse<'a> {
    name: &'a String,
    version: &'a String,
}

fn handler(request: Request) -> Result<impl IntoResponse, VercelError> {
    if request.method() != Method::GET {
        return Ok(Response::builder()
            .status(StatusCode::METHOD_NOT_ALLOWED)
            .body(Body::Empty)?);
    }

    let url = parse_url(&request)?;
    let query_params: QueryParams = (&url).into();
    let npm_package = (&query_params).try_into()?;
    let manager = NpmManager::new(NpmClient::new());
    let package = manager.get_package(&npm_package)?;

    match manager.get_latest_package_version(&package) {
        Ok(version) => mason_registry_api::ok_json(LatestVersionResponse {
            name: &package.name,
            version,
        }),
        Err(err) => mason_registry_api::err_json(err),
    }
}

// Start the runtime with the handler
fn main() -> Result<(), Box<dyn Error>> {
    Ok(lambda!(handler))
}
