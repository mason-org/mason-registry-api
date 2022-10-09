use std::{collections::HashMap, convert::TryFrom};

use vercel_lambda::error::VercelError;

pub mod client;

#[derive(Debug)]
pub struct NpmPackage {
    pub scope: Option<String>,
    pub name: String,
}

impl TryFrom<&url::Url> for NpmPackage {
    type Error = VercelError;

    fn try_from(url: &url::Url) -> Result<Self, Self::Error> {
        let mut query = HashMap::new();
        for (key, val) in url.query_pairs().into_owned() {
            query.insert(key, val);
        }

        match (query.get("scope"), query.get("package")) {
            (Some(scope), Some(name)) if *scope == "_" => Ok(Self {
                scope: None,
                name: name.to_owned(),
            }),
            (Some(scope), Some(name)) => Ok(Self {
                scope: Some(scope.to_owned()),
                name: name.to_owned(),
            }),
            (Some(_), None) | (None, None) | (None, Some(_)) => {
                Err(VercelError::new("Failed to parse npm package from URL."))
            }
        }
    }
}
