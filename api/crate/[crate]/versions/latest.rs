use http::{Method, StatusCode};
use mason_registry_api::{
    crates::{api::CrateResponse, manager::CratesManager},
    parse_url, QueryParams,
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
    let crate_pkg = (&query_params).into();
    let manager = CratesManager::new();

    match manager.get_crate(crate_pkg) {
        Ok(crate_response) => mason_registry_api::ok_json(CrateResponse::from_crate_response(
            crate_response.crate_data.max_version.clone(),
            crate_response,
        )),
        Err(err) => mason_registry_api::err_json(err),
    }
}

// Start the runtime with the handler
fn main() -> Result<(), Box<dyn Error>> {
    Ok(lambda!(handler))
}
