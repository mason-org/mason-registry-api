use serde::Serialize;

use super::client::spec::OpenVSXExtensionDto;

#[derive(Serialize)]
pub struct OpenVSXExtensionResponse {
    pub name: String,
    pub version: String,
}

impl From<OpenVSXExtensionDto> for OpenVSXExtensionResponse {
    fn from(dto: OpenVSXExtensionDto) -> Self {
        Self {
            name: format!("{}/{}", dto.namespace, dto.name),
            version: dto.version,
        }
    }
}
