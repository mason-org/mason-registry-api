use std::convert::TryFrom;

use reqwest::blocking::Response;
use serde::de::DeserializeOwned;

#[derive(Debug)]
pub struct GitHubResponse<T: DeserializeOwned> {
    pub data: T,
    pub links: Option<parse_link_header::LinkMap>,
}

#[derive(Debug)]
pub struct GitHubErrorResponse {
    pub response: Response,
}

impl<T: DeserializeOwned> TryFrom<Response> for GitHubResponse<T> {
    type Error = reqwest::Error;

    fn try_from(value: Response) -> Result<Self, Self::Error> {
        let value = value.error_for_status()?;
        let links = value
            .headers()
            .get("link")
            .and_then(|link| link.to_str().ok())
            .and_then(|link| parse_link_header::parse(link).ok());
        Ok(Self {
            data: value.json()?,
            links,
        })
    }
}
