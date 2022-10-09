use std::convert::TryFrom;

use vercel_lambda::error::VercelError;

use crate::UriQueryParams;

pub mod client;

#[derive(Debug)]
pub struct NpmPackage {
    pub name: String,
}

impl TryFrom<&UriQueryParams> for NpmPackage {
    type Error = VercelError;

    fn try_from(params: &UriQueryParams) -> Result<Self, Self::Error> {
        if let Some(name) = params.params.get("package").and_then(|o| o.to_owned()) {
            return Ok(Self { name });
        }
        Err(VercelError::new(&format!(
            "Failed to parse npm package from {:?}",
            params
        )))
    }
}
