use serde::Serialize;

#[derive(Serialize)]
pub struct CrateResponse {
    pub name: String,
    pub version: String,
}

impl CrateResponse {
    pub fn from_crate_response(
        version: String,
        response: crates_io_api::CrateResponse,
    ) -> CrateResponse {
        CrateResponse {
            name: response.crate_data.name,
            version,
        }
    }
}
