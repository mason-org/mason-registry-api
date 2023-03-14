use crate::QueryParams;

pub mod client;
pub mod errors;
pub mod manager;

#[derive(Debug)]
pub struct NpmPackage {
    pub scope: Option<String>,
    pub name: String,
}

impl From<&QueryParams> for NpmPackage {
    fn from(query: &QueryParams) -> Self {
        match (query.get("scope"), query.get("package")) {
            (Some(scope), Some(name)) if *scope == "_" => Self {
                scope: None,
                name: name.to_owned(),
            },
            (Some(scope), Some(name)) => Self {
                scope: Some(scope.to_owned()),
                name: name.to_owned(),
            },
            (Some(_), None) | (None, None) | (None, Some(_)) => {
                panic!("Failed to parse npm package from URL.")
            }
        }
    }
}
