use std::convert::TryFrom;

use vercel_lambda::error::VercelError;

use crate::QueryParams;

pub mod client;
pub mod errors;
pub mod manager;

#[derive(Debug)]
pub struct NpmPackage {
    pub scope: Option<String>,
    pub name: String,
}

impl TryFrom<&QueryParams> for NpmPackage {
    type Error = VercelError;

    fn try_from(query: &QueryParams) -> Result<Self, Self::Error> {
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
