use crate::QueryParams;

pub mod client;
pub mod errors;
pub mod manager;

#[derive(Debug)]
pub struct PyPiPackage {
    name: String,
}

impl From<&QueryParams> for PyPiPackage {
    fn from(query: &QueryParams) -> Self {
        PyPiPackage {
            name: query
                .get("package")
                .expect("No [package] query param.")
                .to_owned(),
        }
    }
}
