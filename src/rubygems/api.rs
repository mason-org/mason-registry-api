use serde::Serialize;

use super::client::spec::{RubyGemDto, RubyGemVersionDto};

#[derive(Serialize)]
pub struct RubyGemResponse {
    pub name: String,
    pub version: String,
    pub licenses: Vec<String>,
}

impl From<RubyGemDto> for RubyGemResponse {
    fn from(gem_dto: RubyGemDto) -> Self {
        Self {
            name: gem_dto.name,
            version: gem_dto.version,
            licenses: gem_dto.licenses.unwrap_or_else(|| vec![]),
        }
    }
}

impl RubyGemResponse {
    pub fn from_versioned_dto(
        gem_name: String,
        gem_version_dto: RubyGemVersionDto,
    ) -> RubyGemResponse {
        RubyGemResponse {
            name: gem_name,
            version: gem_version_dto.version,
            licenses: gem_version_dto.licenses.unwrap_or_else(|| vec![]),
        }
    }
}
