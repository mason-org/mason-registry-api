pub mod spec;

use std::fmt::Display;

use crate::http::client::{Client, HttpEndpoint};

use self::spec::PackagistPackageResponseEnvelope;

use super::PackagistPackage;

enum PackagistEndpoint<'a> {
    Package(&'a PackagistPackage),
}

impl<'a> HttpEndpoint for PackagistEndpoint<'a> {
    fn as_full_url(&self) -> String {
        format!("https://repo.packagist.org/{}", self)
    }
}

impl<'a> Display for PackagistEndpoint<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PackagistEndpoint::Package(pkg) => {
                f.write_fmt(format_args!("p2/{}/{}.json", pkg.vendor, pkg.name))
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
            client: Client::new(None),
        }
    }

    pub fn fetch_package(
        &self,
        package: &PackagistPackage,
    ) -> Result<PackagistPackageResponseEnvelope, reqwest::Error> {
        self.client.get(PackagistEndpoint::Package(package))?.json()
    }
}
