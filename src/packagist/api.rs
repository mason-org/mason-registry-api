use serde::Serialize;

use super::client::spec::PackagistPackageDto;

#[derive(Serialize)]
pub struct PackagistResponse {
    pub name: String,
    pub version: String,
}

impl PackagistResponse {
    pub fn from_packagist_package_dto(name: String, dto: PackagistPackageDto) -> Self {
        Self {
            name,
            version: dto.version,
        }
    }
}
