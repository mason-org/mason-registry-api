use std::convert::TryFrom;

use reqwest::blocking::Response;
use serde::de::DeserializeOwned;
use vercel_lambda::error::VercelError;

pub struct GitHubResponse<T: DeserializeOwned> {
    pub data: T,
    pub links: Option<parse_link_header::LinkMap>,
}

impl<T: DeserializeOwned> TryFrom<Response> for GitHubResponse<T> {
    type Error = VercelError;

    fn try_from(value: Response) -> Result<Self, Self::Error> {
        let links = value
            .headers()
            .get("link")
            .and_then(|link| link.to_str().ok())
            .and_then(|link| parse_link_header::parse(link).ok());
        Ok(Self {
            data: value
                .json()
                .map_err(|_| VercelError::new("Failed to deserialize JSON."))?,
            links,
        })
    }
}
