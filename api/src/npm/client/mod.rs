pub mod spec;

use std::fmt::Display;

use reqwest::header::{HeaderMap, ACCEPT};

use crate::http::client::{Client, HttpEndpoint};

use self::spec::NpmAbbrevPackageDto;

use super::NpmPackage;

enum NpmEndpoint<'a> {
    Package(&'a NpmPackage),
}

impl<'a> HttpEndpoint for NpmEndpoint<'a> {
    fn as_full_url(&self) -> String {
        format!("https://registry.npmjs.com/{}", self)
    }
}

impl<'a> Display for NpmEndpoint<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NpmEndpoint::Package(pkg) => match pkg {
                NpmPackage { scope: None, name } => f.write_fmt(format_args!("{}", name)),
                NpmPackage {
                    scope: Some(scope),
                    name,
                } => f.write_fmt(format_args!("{}/{}", scope, name)),
            },
        }
    }
}

pub struct NpmClient {
    client: Client,
}

impl NpmClient {
    pub fn new() -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(
            ACCEPT,
            "application/vnd.npm.install-v1+json; q=1.0, application/json; q=0.8"
                .parse()
                .unwrap(),
        );
        NpmClient {
            client: Client::new(Some(headers)),
        }
    }

    pub fn fetch_package(
        &self,
        package: &NpmPackage,
    ) -> Result<NpmAbbrevPackageDto, reqwest::Error> {
        self.client.get(NpmEndpoint::Package(package))?.json()
    }
}
