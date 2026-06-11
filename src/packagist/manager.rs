use std::collections::LinkedList;

use super::{
    PackagistPackage,
    client::{PackagistClient, spec::PackagistPackageDto},
    errors::PackagistError,
};

pub struct PackagistManager {
    client: PackagistClient,
}

impl PackagistManager {
    pub fn new(client: PackagistClient) -> Self {
        Self { client }
    }

    async fn resolve_package_versions(
        &self,
        package: &PackagistPackage,
    ) -> Result<LinkedList<PackagistPackageDto>, PackagistError> {
        self.client
            .fetch_package(package)
            .await?
            .packages
            .remove(&package.to_string())
            .ok_or_else(|| PackagistError::ResourceNotFound { source: None })
    }

    pub async fn get_package(
        &self,
        package: &PackagistPackage,
    ) -> Result<PackagistPackageDto, PackagistError> {
        self.resolve_package_versions(package)
            .await?
            .pop_front()
            .ok_or_else(|| PackagistError::ResourceNotFound { source: None })
    }

    pub async fn get_package_version(
        &self,
        package: &PackagistPackage,
        version: &str,
    ) -> Result<PackagistPackageDto, PackagistError> {
        self.resolve_package_versions(package)
            .await?
            .into_iter()
            .find(|v| v.version == version)
            .ok_or_else(|| PackagistError::ResourceNotFound { source: None })
    }

    /// Returns all package versions in DESCENDING order.
    pub async fn get_all_package_versions(
        &self,
        package: &PackagistPackage,
    ) -> Result<Vec<String>, PackagistError> {
        Ok(self
            .resolve_package_versions(package)
            .await?
            .into_iter()
            .map(|v| v.version)
            .collect())
    }
}
