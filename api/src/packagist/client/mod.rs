pub mod spec;

use std::fmt::Display;

use reqwest::{
    blocking::{Client, Response},
    header::{HeaderMap, ACCEPT, USER_AGENT},
};
use serde::Serialize;

use self::spec::PackagistPackageResponseEnvelope;

use super::PackagistPackage;

enum PackagistEndpoint<'a> {
    Package(&'a PackagistPackage),
}

impl<'a> PackagistEndpoint<'a> {
    fn as_full_url(&self) -> String {
        format!("https://repo.packagist.org{}", self)
    }
}

impl<'a> Display for PackagistEndpoint<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PackagistEndpoint::Package(pkg) => {
                f.write_fmt(format_args!("/p2/{}/{}.json", pkg.vendor, pkg.name))
            }
        }
    }
}

pub struct PackagistClient {
    client: Client,
}

impl PackagistClient {
    pub fn new() -> Self {
        PackagistClient {
            client: reqwest::blocking::Client::new(),
        }
    }

    fn headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(
            ACCEPT,
            "application/json"
                .parse()
                .unwrap(),
        );
        headers.insert(
            USER_AGENT,
            "mason-registry-api (+https://github.com/mason-org/mason-registry-api)"
                .parse()
                .unwrap(),
        );
        headers
    }

    fn get(&self, endpoint: PackagistEndpoint) -> Result<Response, reqwest::Error> {
        self.client
            .get(endpoint.as_full_url())
            .headers(self.headers())
            .send()?
            .error_for_status()
    }

    #[allow(dead_code)]
    fn post<Json: Serialize>(
        &self,
        endpoint: PackagistEndpoint,
        json: &Json,
    ) -> Result<Response, reqwest::Error> {
        self.client
            .post(endpoint.as_full_url())
            .headers(self.headers())
            .json(json)
            .send()?
            .error_for_status()
    }

    pub fn fetch_package(
        &self,
        package: &PackagistPackage,
    ) -> Result<PackagistPackageResponseEnvelope, reqwest::Error> {
        self.get(PackagistEndpoint::Package(package))?.json()
    }
}
