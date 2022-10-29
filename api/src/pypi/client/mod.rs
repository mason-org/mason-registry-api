use std::fmt::Display;

use reqwest::{
    blocking::{Client, Response},
    header::{HeaderMap, ACCEPT, USER_AGENT},
};
use serde::Serialize;

use self::spec::PyPiProjectDto;

use super::PyPiPackage;

pub mod spec;

pub enum PyPiEndpoint<'a> {
    Project(&'a PyPiPackage),
    ProjectVersion(&'a PyPiPackage, &'a str),
}

impl<'a> PyPiEndpoint<'a> {
    fn as_full_url(&self) -> String {
        format!("https://pypi.org/pypi/{}", self)
    }
}

impl<'a> Display for PyPiEndpoint<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PyPiEndpoint::Project(project) => {
                f.write_fmt(format_args!("{}/json", project.name))
            }
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
            client: reqwest::blocking::Client::new(),
        }
    }

    fn headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT, "application/json".parse().unwrap());
        headers.insert(
            USER_AGENT,
            "mason-registry-api (+https://github.com/williamboman/mason-registry-api)"
                .parse()
                .unwrap(),
        );
        headers
    }

    fn get(&self, endpoint: PyPiEndpoint) -> Result<Response, reqwest::Error> {
        let endd = endpoint.as_full_url();
        self.client
            .get(endd)
            .headers(self.headers())
            .send()
    }

    #[allow(dead_code)]
    fn post<Json: Serialize>(
        &self,
        endpoint: PyPiEndpoint,
        json: &Json,
    ) -> Result<Response, reqwest::Error> {
        self.client
            .post(endpoint.as_full_url())
            .headers(self.headers())
            .json(json)
            .send()
    }

    pub fn fetch_project(&self, project: &PyPiPackage) -> Result<PyPiProjectDto, reqwest::Error> {
        self.get(PyPiEndpoint::Project(project))?.json()
    }

    pub fn fetch_project_version(
        &self,
        project: &PyPiPackage,
        version: &str,
    ) -> Result<PyPiProjectDto, reqwest::Error> {
        self.get(PyPiEndpoint::ProjectVersion(project, version))?
            .json()
    }
}
