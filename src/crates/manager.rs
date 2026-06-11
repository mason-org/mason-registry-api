use std::time::Duration;

use super::{Crate, errors::CratesError};

pub struct CratesManager {
    client: crates_io_api::AsyncClient,
}

impl CratesManager {
    pub fn new() -> Self {
        Self {
            client: crates_io_api::AsyncClient::new(
                "mason-registry-api (+https://github.com/mason-org/mason-registry-api)",
                Duration::from_secs(1),
            )
            .expect("Failed to instantiate SyncClient."),
        }
    }

    pub async fn get_crate(
        &self,
        crate_pkg: Crate,
    ) -> Result<crates_io_api::CrateResponse, CratesError> {
        Ok(self.client.get_crate(&crate_pkg.name).await?)
    }

    /// Returns all crate versions in DESCENDING order.
    pub async fn get_all_crate_versions(
        &self,
        crate_pkg: Crate,
    ) -> Result<Vec<String>, CratesError> {
        let crate_response = self.get_crate(crate_pkg).await?;
        return Ok(crate_response.versions.into_iter().map(|v| v.num).collect());
    }

    pub async fn get_crate_version(
        &self,
        crate_pkg: Crate,
        version: &str,
    ) -> Result<crates_io_api::CrateResponse, CratesError> {
        let crate_response = self.get_crate(crate_pkg).await?;
        if crate_response.versions.iter().any(|v| v.num == version) {
            Ok(crate_response)
        } else {
            Err(CratesError::ResourceNotFound { source: None })
        }
    }
}
