use super::{
    RubyGemPackage,
    client::{
        RubyGemsClient,
        spec::{RubyGemDto, RubyGemVersionDto},
    },
    errors::RubyGemsError,
};

pub struct RubyGemsManager {
    client: RubyGemsClient,
}

impl RubyGemsManager {
    pub fn new(client: RubyGemsClient) -> Self {
        Self { client }
    }

    pub async fn get_gem(&self, gem: &RubyGemPackage) -> Result<RubyGemDto, RubyGemsError> {
        Ok(self.client.fetch_gem(gem).await?)
    }

    pub async fn get_gem_version(
        &self,
        gem: &RubyGemPackage,
        version: &str,
    ) -> Result<RubyGemVersionDto, RubyGemsError> {
        let gem_versions = self.client.fetch_gem_versions(gem).await?;
        gem_versions
            .into_iter()
            .find(|gem| gem.version == version)
            .ok_or_else(|| RubyGemsError::ResourceNotFound { source: None })
    }

    /// Returns all package versions in DESCENDING order.
    pub async fn get_all_gem_versions(
        &self,
        gem: &RubyGemPackage,
    ) -> Result<Vec<String>, RubyGemsError> {
        Ok(self
            .client
            .fetch_gem_versions(gem)
            .await?
            .into_iter()
            .filter_map(|gem| {
                if !gem.prerelease {
                    Some(gem.version)
                } else {
                    None
                }
            })
            .collect())
    }
}
