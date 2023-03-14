use std::fmt::Display;

use crate::http::client::{Client, HttpEndpoint};

use self::spec::{PyPiProjectDto, PyPiProjectVersionedDto};

use super::PyPiPackage;

pub mod spec;

pub enum PyPiEndpoint<'a> {
    Project(&'a PyPiPackage),
    ProjectVersion(&'a PyPiPackage, &'a str),
}

impl<'a> HttpEndpoint for PyPiEndpoint<'a> {
    fn as_full_url(&self) -> String {
        format!("https://pypi.org/pypi/{}", self)
    }
}

impl<'a> Display for PyPiEndpoint<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PyPiEndpoint::Project(project) => f.write_fmt(format_args!("{}/json", project.name)),
            PyPiEndpoint::ProjectVersion(project, version) => {
                f.write_fmt(format_args!("{}/{}/json", project.name, version))
            }
        }
    }
}

pub struct PyPiClient {
    client: Client,
}

impl PyPiClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(None),
        }
    }

    pub fn fetch_project(&self, project: &PyPiPackage) -> Result<PyPiProjectDto, reqwest::Error> {
        self.client.get(PyPiEndpoint::Project(project))?.json()
    }

    pub fn fetch_project_version(
        &self,
        project: &PyPiPackage,
        version: &str,
    ) -> Result<PyPiProjectVersionedDto, reqwest::Error> {
        self.client
            .get(PyPiEndpoint::ProjectVersion(project, version))?
            .json()
    }
}
