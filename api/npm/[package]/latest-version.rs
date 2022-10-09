use http::{Method, StatusCode};
use mason_registry_api::{
    get_query_params,
    npm::{
        client::{
            spec::{NpmAbbrevPackageDto, NpmDistTag},
            NpmClient,
        },
        NpmPackage,
    },
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

    let query_params = get_query_params(&request)?;
    let package: NpmPackage = (&query_params).try_into()?;
    let client = NpmClient::new();

    let response: LatestVersionResponse = client.fetch_package(&package)?.try_into()?;

    mason_registry_api::api::json(response)
}

// Start the runtime with the handler
fn main() -> Result<(), Box<dyn Error>> {
    Ok(lambda!(handler))
}
