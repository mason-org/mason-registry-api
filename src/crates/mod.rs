use crate::QueryParams;

pub mod api;
pub mod errors;
pub mod manager;

#[derive(Debug)]
pub struct Crate {
    pub name: String,
}

impl From<&QueryParams> for Crate {
    fn from(query: &QueryParams) -> Self {
        Self {
            name: query
                .get("crate")
                .expect("No [crate] query param")
                .to_owned(),
        }
    }
}
