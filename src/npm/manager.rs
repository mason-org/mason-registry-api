use std::cmp::Ordering;

use super::{
    client::{
        spec::{NpmAbbrevPackageDto, NpmAbbrevPackageVersionDto, NpmDistTag},
        NpmClient,
    },
    errors::NpmError,
    NpmPackage,
};

fn semver_sort_desc(a: &String, b: &String) -> Ordering {
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

    pub fn get_package(&self, package: &NpmPackage) -> Result<NpmAbbrevPackageDto, NpmError> {
        Ok(self.client.fetch_package(&package)?)
    }

    pub fn get_package_version<'a>(
        &self,
        package: &'a NpmAbbrevPackageDto,
        version: &str,
    ) -> Result<&'a NpmAbbrevPackageVersionDto, NpmError> {
        package
            .versions
            .get(version)
            .ok_or_else(|| NpmError::ResourceNotFound { source: None })
    }

    pub fn get_latest_package_version<'a>(
        &self,
        package: &'a NpmAbbrevPackageDto,
    ) -> Result<&'a NpmAbbrevPackageVersionDto, NpmError> {
        let latest_version = package
            .dist_tags
            .get(&NpmDistTag::Latest)
            .ok_or_else(|| NpmError::ResourceNotFound { source: None })?;
        self.get_package_version(package, latest_version)
    }

    /// Returns all package versions in DESCENDING order.
    pub fn get_all_package_versions(&self, package: &NpmPackage) -> Result<Vec<String>, NpmError> {
        let npm_package = self.get_package(package)?;
        let mut versions: Vec<String> = npm_package.versions.into_keys().collect();
        // https://github.com/npm/cli/blob/32336f6efe06bd52de1dc67c0f812d4705533ef2/lib/commands/view.js#L54
        versions.sort_by(semver_sort_desc);
        Ok(versions)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

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
        input.sort_by(semver_sort_desc);
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

    #[test]
    fn should_return_latest_package_version() -> Result<(), NpmError> {
        let manager = NpmManager::new(NpmClient::new());
        let package = NpmAbbrevPackageDto {
            name: "foobar".to_owned(),
            dist_tags: HashMap::from([
                (NpmDistTag::Next, "14.0.0-pre.1".to_owned()),
                (NpmDistTag::Latest, "13.3.7".to_owned()),
            ]),
            versions: HashMap::from([(
                "13.3.7".to_owned(),
                NpmAbbrevPackageVersionDto {
                    name: "foobar".to_owned(),
                    version: "13.3.7".to_owned(),
                },
            )]),
        };
        let latest_version = manager.get_latest_package_version(&package)?;
        assert_eq!("13.3.7".to_owned(), latest_version.version);
        Ok(())
    }
}
