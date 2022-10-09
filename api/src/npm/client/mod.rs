pub mod spec;

use std::fmt::Display;

use reqwest::{
    blocking::{Client, Response},
    header::{HeaderMap, ACCEPT, USER_AGENT},
};
use serde::Serialize;
use vercel_lambda::error::VercelError;

use self::spec::NpmAbbrevPackageDto;

use super::NpmPackage;

enum NpmEndpoint<'a> {
    Package(&'a NpmPackage),
}

impl<'a> NpmEndpoint<'a> {
    fn as_full_url(&self) -> String {
        format!("https://registry.npmjs.com{}", self)
    }
}

impl<'a> Display for NpmEndpoint<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NpmEndpoint::Package(pkg) => match pkg {
                NpmPackage { scope: None, name } => f.write_fmt(format_args!("/{}", name)),
                NpmPackage {
                    scope: Some(scope),
                    name,
                } => f.write_fmt(format_args!("/{}/{}", scope, name)),
            },
        }
    }
}

pub struct NpmClient {
    client: Client,
}

impl NpmClient {
    pub fn new() -> Self {
        NpmClient {
            client: reqwest::blocking::Client::new(),
        }
    }

    fn headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        // Accept abbreviated responses, see https://github.com/npm/registry/blob/master/docs/responses/package-metadata.md
        headers.insert(
            ACCEPT,
            "application/vnd.npm.install-v1+json; q=1.0, application/json; q=0.8"
                .parse()
                .unwrap(),
        );
        headers.insert(
            USER_AGENT,
            "mason-registry-api (+https://github.com/williamboman/mason-registry-api)"
                .parse()
                .unwrap(),
        );
        headers
    }

    fn get(&self, endpoint: NpmEndpoint) -> Result<Response, reqwest::Error> {
        self.client
            .get(endpoint.as_full_url())
            .headers(self.headers())
            .send()
    }

    #[allow(dead_code)]
    fn post<Json: Serialize>(
        &self,
        endpoint: NpmEndpoint,
        json: &Json,
    ) -> Result<Response, reqwest::Error> {
        self.client
            .post(endpoint.as_full_url())
            .headers(self.headers())
            .json(json)
            .send()
    }

    pub fn fetch_package(&self, package: &NpmPackage) -> Result<NpmAbbrevPackageDto, VercelError> {
        self.get(NpmEndpoint::Package(package))
            .map_err(|_| VercelError::new("Failed to fetch npm package."))?
            .json()
            .map_err(|_| VercelError::new("Failed to parse JSON."))
    }
}
