use crate::QueryParams;

pub mod api;
pub mod client;
pub mod errors;
pub mod manager;

#[derive(Debug)]
pub struct OpenVSXExtension {
    pub namespace: String,
    pub extension: String,
}

impl From<&QueryParams> for OpenVSXExtension {
    fn from(query: &QueryParams) -> Self {
        Self {
            namespace: query
                .get("namespace")
                .expect("No [namespace] query param")
                .to_owned(),
            extension: query
                .get("extension")
                .expect("No [extension] query param")
                .to_owned(),
        }
    }
}
