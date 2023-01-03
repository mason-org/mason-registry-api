use crate::QueryParams;

pub mod client;
pub mod errors;
pub mod manager;

#[derive(Debug)]
pub struct GolangPackage {
    pub name: String,
}

impl From<&QueryParams> for GolangPackage {
    fn from(query: &QueryParams) -> Self {
        Self {
            name: query
                .get("package")
                .expect("No [package] query param")
                .to_owned(),
        }
    }
}
