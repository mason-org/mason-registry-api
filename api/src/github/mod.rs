pub mod client;

use std::{collections::HashMap, convert::TryFrom, fmt::Display};

use vercel_lambda::error::VercelError;

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

impl TryFrom<&url::Url> for GitHubRepo {
    type Error = VercelError;

    fn try_from(url: &url::Url) -> Result<Self, Self::Error> {
        let mut query = HashMap::new();
        for (key, val) in url.query_pairs().into_owned() {
            query.insert(key, val);
        }

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
