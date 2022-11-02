use std::fmt::Display;

use crate::QueryParams;

pub mod api;
pub mod client;
pub mod errors;
pub mod manager;

#[derive(Debug)]
pub struct PackagistPackage {
    pub vendor: String,
    pub name: String,
}

impl From<&QueryParams> for PackagistPackage {
    fn from(query: &QueryParams) -> Self {
        match (query.get("vendor"), query.get("package")) {
            (Some(scope), Some(name)) => Self {
                vendor: scope.to_owned(),
                name: name.to_owned(),
            },
            (Some(_), None) | (None, None) | (None, Some(_)) => {
                panic!("Failed to parse npm package from URL.")
            }
        }
    }
}

impl Display for PackagistPackage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}/{}", self.vendor, self.name))
    }
}
