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

const VERSIONS_OFFSET: u64 = 50;

impl OpenVSXManager {
    pub fn new(client: OpenVSXClient) -> Self {
        Self { client }
    }

    pub fn get_extension(
        &self,
        extension: &OpenVSXExtension,
    ) -> Result<OpenVSXExtensionDto, OpenVSXError> {
        Ok(self.client.fetch_extension(extension)?)
    }

    /// Returns all extension versions in DESCENDING order.
    pub fn get_all_versions(
        &self,
        extension: &OpenVSXExtension,
    ) -> Result<Vec<String>, OpenVSXError> {
        let mut unsorted_versions: Vec<String> = vec![];

        let mut cursor = 0;
        let mut total_size;

        loop {
            let response =
                self.client
                    .fetch_extension_versions(extension, VERSIONS_OFFSET, cursor)?;
            total_size = response.total_size;
            cursor += VERSIONS_OFFSET;
            unsorted_versions.append(&mut response.versions.into_keys().collect());

            if cursor >= total_size {
                break;
            }
        }

        unsorted_versions.sort_by(semver_sort_desc);
        Ok(unsorted_versions)
    }
}
