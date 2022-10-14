use std::cmp::Ordering;

use vercel_lambda::error::VercelError;

use super::{
    client::{spec::NpmAbbrevPackageDto, NpmClient},
    NpmPackage,
};

fn semver_sort(a: &String, b: &String) -> Ordering {
    let a_semver = a.parse::<semver::Version>();
    let b_semver = b.parse::<semver::Version>();
    if let (Ok(a), Ok(b)) = (a_semver, b_semver) {
        return b.cmp(&a);
    }
    Ordering::Equal
}

pub struct NpmManager {
    client: NpmClient,
}

impl NpmManager {
    pub fn new(client: NpmClient) -> Self {
        Self { client }
    }

    pub fn get_package(&self, package: &NpmPackage) -> Result<NpmAbbrevPackageDto, VercelError> {
        self.client.fetch_package(&package)
    }

    pub fn get_all_package_versions(
        &self,
        package: &NpmPackage,
    ) -> Result<Vec<String>, VercelError> {
        let npm_package = self.get_package(package)?;
        let mut versions: Vec<String> = npm_package.versions.into_keys().collect();
        // https://github.com/npm/cli/blob/32336f6efe06bd52de1dc67c0f812d4705533ef2/lib/commands/view.js#L54
        versions.sort_by(semver_sort);
        Ok(versions)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_order_by_semver_version_desc() {
        let mut input: Vec<String> = vec![
            "3.0.0",
            "3.0.0-rc.1",
            "3.1.0",
            "3.0.0-alpha.1",
            "0.0.1",
            "3.0.0-alpha.2",
            "2.0.0",
            "3.10.0",
        ]
        .into_iter()
        .map(ToOwned::to_owned)
        .collect();
        input.sort_by(semver_sort);
        assert_eq!(
            vec![
                "3.10.0",
                "3.1.0",
                "3.0.0",
                "3.0.0-rc.1",
                "3.0.0-alpha.2",
                "3.0.0-alpha.1",
                "2.0.0",
                "0.0.1",
            ],
            input
        )
    }
}
