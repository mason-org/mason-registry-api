use crate::QueryParams;

pub mod api;
pub mod client;
pub mod errors;
pub mod manager;

#[derive(Debug)]
pub struct RubyGemPackage {
    pub name: String,
}

impl From<&QueryParams> for RubyGemPackage {
    fn from(query: &QueryParams) -> Self {
        RubyGemPackage {
            name: query.get("gem").expect("No [gem] query param.").to_owned(),
        }
    }
}
