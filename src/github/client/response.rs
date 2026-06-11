use reqwest::Response;
use serde::de::DeserializeOwned;

#[derive(Debug)]
pub struct GitHubResponse<T: DeserializeOwned> {
    pub data: T,
    pub links: Option<parse_link_header::LinkMap>,
}

impl<T: DeserializeOwned> GitHubResponse<T> {
    pub async fn from_response(value: Response) -> Result<Self, reqwest::Error> {
        let value = value.error_for_status()?;
        let links = value
            .headers()
            .get("link")
            .and_then(|link| link.to_str().ok())
            .and_then(|link| parse_link_header::parse(link).ok());
        Ok(Self {
            data: value.json().await?,
            links,
        })
    }
}

#[derive(Debug)]
pub struct GitHubErrorResponse {
    pub response: Response,
}
