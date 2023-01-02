use std::time::Duration;

use super::{errors::CratesError, Crate};

pub struct CratesManager {
    client: crates_io_api::SyncClient,
}

impl CratesManager {
    pub fn new() -> Self {
        Self {
            client: crates_io_api::SyncClient::new(
                "mason-registry-api (+https://github.com/mason-org/mason-registry-api)",
                Duration::from_secs(1),
            )
            .expect("Failed to instantiate SyncClient."),
        }
    }

    pub fn get_crate(&self, crate_pkg: Crate) -> Result<crates_io_api::CrateResponse, CratesError> {
        Ok(self.client.get_crate(&crate_pkg.name)?)
    }

    pub fn get_all_crate_versions(&self, crate_pkg: Crate) -> Result<Vec<String>, CratesError> {
        let crate_response = self.get_crate(crate_pkg)?;
        return Ok(crate_response.versions.into_iter().map(|v| v.num).collect());
    }

    pub fn get_crate_version(
        &self,
        crate_pkg: Crate,
        version: &str,
    ) -> Result<crates_io_api::CrateResponse, CratesError> {
        let crate_response = self.get_crate(crate_pkg)?;
        if crate_response.versions.iter().any(|v| v.num == version) {
            Ok(crate_response)
        } else {
            Err(CratesError::ResourceNotFound { source: None })
        }
    }
}
