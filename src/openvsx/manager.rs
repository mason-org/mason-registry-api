use std::cmp::Ordering;

use super::{
    client::{spec::OpenVSXExtensionDto, OpenVSXClient},
    errors::OpenVSXError,
    OpenVSXExtension,
};

pub struct OpenVSXManager {
    client: OpenVSXClient,
}

fn semver_sort_desc(a: &String, b: &String) -> Ordering {
    let a_semver = a.strip_prefix("v").unwrap_or(a).parse::<semver::Version>();
    let b_semver = b.strip_prefix("v").unwrap_or(b).parse::<semver::Version>();
    if let (Ok(a), Ok(b)) = (&a_semver, &b_semver) {
        return b.cmp(a);
    }
    Ordering::Equal
}

impl OpenVSXManager {
    pub fn new(client: OpenVSXClient) -> Self {
        Self { client }
    }

    pub fn get_extension(
        &self,
        extension: &OpenVSXExtension,
    ) -> Result<OpenVSXExtensionDto, OpenVSXError> {
        Ok(self.client.fetch_latest_extension_version(extension)?)
    }

    /// Returns all extension versions in DESCENDING order.
    pub fn get_all_versions(
        &self,
        extension: &OpenVSXExtension,
    ) -> Result<Vec<String>, OpenVSXError> {
        let mut unsorted_versions: Vec<String> = self
            .client
            .fetch_extension_versions(extension)?
            .versions
            .into_keys()
            .collect();

        unsorted_versions.sort_by(semver_sort_desc);
        Ok(unsorted_versions)
    }
}
