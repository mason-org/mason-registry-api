pub mod client;

use std::{convert::TryFrom, fmt::Display};

use vercel_lambda::error::VercelError;

use crate::UriQueryParams;

#[derive(Debug)]
pub struct GitHubRepo {
    pub owner: String,
    pub name: String,
}

impl Display for GitHubRepo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.owner, self.name)
    }
}

impl TryFrom<UriQueryParams> for GitHubRepo {
    type Error = VercelError;

    fn try_from(params: UriQueryParams) -> Result<Self, Self::Error> {
        if let (Some(owner), Some(name)) = (
            params.params.get("owner").and_then(|o| o.to_owned()),
            params.params.get("name").and_then(|o| o.to_owned()),
        ) {
            return Ok(Self { owner, name });
        }
        Err(VercelError::new(&format!(
            "Failed to parse repo from {:?}",
            params
        )))
    }
}
