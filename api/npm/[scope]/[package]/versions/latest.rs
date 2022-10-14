use http::{Method, StatusCode};
use mason_registry_api::{
    npm::{
        client::{
            spec::{NpmAbbrevPackageDto, NpmDistTag},
            NpmClient,
        },
        manager::NpmManager,
    },
    parse_url,
};
use serde::Serialize;
use std::{
    convert::{TryFrom, TryInto},
    error::Error,
};

use vercel_lambda::{error::VercelError, lambda, Body, IntoResponse, Request, Response};

#[derive(Serialize)]
struct LatestVersionResponse {
    name: String,
    version: String,
}

impl TryFrom<NpmAbbrevPackageDto> for LatestVersionResponse {
    type Error = VercelError;

    fn try_from(value: NpmAbbrevPackageDto) -> Result<Self, Self::Error> {
        Ok(Self {
            name: value.name,
            version: value
                .dist_tags
                .get(&NpmDistTag::Latest)
                .ok_or_else(|| VercelError::new("Unable to find latest dist-tag."))?
                .to_string(),
        })
    }
}

fn handler(request: Request) -> Result<impl IntoResponse, VercelError> {
    if request.method() != Method::GET {
        return Ok(Response::builder()
            .status(StatusCode::METHOD_NOT_ALLOWED)
            .body(Body::Empty)?);
    }

    let url = parse_url(&request)?;
    let npm_package = (&url).try_into()?;
    let manager = NpmManager::new(NpmClient::new());
    let package = manager.get_package(&npm_package)?;
    mason_registry_api::json::<LatestVersionResponse>(package.try_into()?)
}

// Start the runtime with the handler
fn main() -> Result<(), Box<dyn Error>> {
    Ok(lambda!(handler))
}
