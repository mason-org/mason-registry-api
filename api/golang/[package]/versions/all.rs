use http::{Method, StatusCode};
use mason_registry_api::{
    golang::{client::GolangClient, manager::GolangManager},
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
    let golang_pkg = (&query_params).into();
    let manager = GolangManager::new(GolangClient::new());

    match manager.get_all_versions(&golang_pkg) {
        Ok(versions) => mason_registry_api::ok_json(versions),
        Err(err) => mason_registry_api::err_json(err),
    }
}

// Start the runtime with the handler
fn main() -> Result<(), Box<dyn Error>> {
    Ok(lambda!(handler))
}
