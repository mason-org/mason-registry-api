use serde::Serialize;

#[derive(Serialize)]
pub struct CrateResponse {
    pub name: String,
    pub version: String,
}

impl From<crates_io_api::CrateResponse> for CrateResponse {
    fn from(value: crates_io_api::CrateResponse) -> Self {
        Self {
            name: value.crate_data.name,
            version: value.crate_data.max_version
        }
    }
}
