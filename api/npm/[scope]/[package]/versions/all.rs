use http::{Method, StatusCode};
use mason_registry_api::{
    npm::{client::NpmClient, manager::NpmManager},
    parse_url,
};

use std::{convert::TryInto, error::Error};

use vercel_lambda::{error::VercelError, lambda, Body, IntoResponse, Request, Response};

fn handler(request: Request) -> Result<impl IntoResponse, VercelError> {
    if request.method() != Method::GET {
        return Ok(Response::builder()
            .status(StatusCode::METHOD_NOT_ALLOWED)
            .body(Body::Empty)?);
    }

    let url = parse_url(&request)?;
    let npm_package = (&url).try_into()?;
    let manager = NpmManager::new(NpmClient::new());
    let versions = manager.get_all_package_versions(&npm_package)?;
    mason_registry_api::json(versions)
}

// Start the runtime with the handler
fn main() -> Result<(), Box<dyn Error>> {
    Ok(lambda!(handler))
}
