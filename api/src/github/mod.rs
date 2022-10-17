pub mod client;
pub mod errors;
pub mod manager;

use std::{convert::TryFrom, fmt::Display, str::FromStr};

use vercel_lambda::error::VercelError;

use crate::QueryParams;

#[derive(Debug)]
pub struct GitHubRepo {
    pub owner: String,
    pub name: String,
}

pub struct GitHubReleaseTag(String);

impl Display for GitHubReleaseTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl FromStr for GitHubReleaseTag {
    type Err = VercelError;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        Ok(Self(str.to_owned()))
    }
}

impl Display for GitHubRepo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.owner, self.name)
    }
}

impl TryFrom<&QueryParams> for GitHubRepo {
    type Error = VercelError;

    fn try_from(query: &QueryParams) -> Result<Self, Self::Error> {
        match (query.get("owner"), query.get("name")) {
            (Some(owner), Some(name)) => Ok(Self {
                owner: owner.to_owned(),
                name: name.to_owned(),
            }),
            (Some(_), None) | (None, None) | (None, Some(_)) => {
                Err(VercelError::new("Failed to parse npm package from URL."))
            }
        }
    }
}
