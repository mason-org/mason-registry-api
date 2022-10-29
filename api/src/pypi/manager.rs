use std::cmp::Ordering;

use super::{
    client::{spec::PyPiProjectDto, PyPiClient},
    errors::PyPiError,
    PyPiPackage,
};

pub struct PyPiManager {
    client: PyPiClient,
}

pub enum PyPiPackageVersion<'a> {
    Version(&'a String),
    Latest,
}

fn maybe_semver_sort_desc(a: &String, b: &String) -> Ordering {
    let a_semver = a.parse::<semver::Version>();
    let b_semver = b.parse::<semver::Version>();
    if let (Ok(a), Ok(b)) = (a_semver, b_semver) {
        b.cmp(&a)
    } else {
        b.cmp(a)
    }
}

impl PyPiManager {
    pub fn new(client: PyPiClient) -> Self {
        Self { client }
    }

    pub fn get_project(
        &self,
        package: &PyPiPackage,
        version: PyPiPackageVersion,
    ) -> Result<PyPiProjectDto, PyPiError> {
        match version {
            PyPiPackageVersion::Latest => Ok(self.client.fetch_project(package)?),
            PyPiPackageVersion::Version(version) => {
                Ok(self.client.fetch_project_version(package, version)?)
            }
        }
    }

    /// Returns all package versions in DESCENDING order.
    /// Ordering should not be relied upon as it does not strictly follow pip's version ordering.
    pub fn get_all_package_versions(
        &self,
        package: &PyPiPackage,
    ) -> Result<Vec<String>, PyPiError> {
        let project = self.client.fetch_project(package)?;
        let mut versions: Vec<String> = project.releases.into_keys().into_iter().collect();
        // This is not at all according to pip's version sorting [1], but it makes the vector nicer to the eye.
        // [1]: https://github.com/pypa/pip/blob/d6e333fb636424d7dca15f4e8aa61cdaab9cdd31/src/pip/_vendor/packaging/version.py#L223-L288
        versions.sort_by(maybe_semver_sort_desc);
        Ok(versions)
    }
}
