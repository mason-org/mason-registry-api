use std::cmp::Ordering;

use super::{client::GolangClient, errors::GolangError, GolangPackage};

pub struct GolangManager {
    client: GolangClient,
}

fn semver_sort_desc(a: &String, b: &String) -> Ordering {
    let a_semver = a.strip_prefix("v").unwrap_or(a).parse::<semver::Version>();
    let b_semver = b.strip_prefix("v").unwrap_or(b).parse::<semver::Version>();
    if let (Ok(a), Ok(b)) = (&a_semver, &b_semver) {
        return b.cmp(a);
    }
    Ordering::Equal
}

impl GolangManager {
    pub fn new(client: GolangClient) -> Self {
        Self { client }
    }

    /// Returns all package versions in DESCENDING order.
    pub fn get_all_versions(&self, package: &GolangPackage) -> Result<Vec<String>, GolangError> {
        let mut unsorted_versions = self.client.fetch_package_versions(package)?;
        unsorted_versions.sort_by(semver_sort_desc);
        Ok(unsorted_versions)
    }
}
