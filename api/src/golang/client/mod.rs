use std::fmt::Display;

use crate::http::client::{Client, HttpEndpoint};

use super::GolangPackage;

pub struct GolangClient {
    client: Client,
}

enum GolangEndpoint<'a> {
    VersionsList(&'a GolangPackage),
}

impl<'a> HttpEndpoint for GolangEndpoint<'a> {
    fn as_full_url(&self) -> String {
        format!("https://proxy.golang.org/{}", self)
    }
}

impl<'a> Display for GolangEndpoint<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GolangEndpoint::VersionsList(pkg) => f.write_fmt(format_args!("{}/@v/list", pkg.name)),
        }
    }
}

impl GolangClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(None),
        }
    }

    pub fn fetch_package_versions(
        &self,
        package: &GolangPackage,
    ) -> Result<Vec<String>, reqwest::Error> {
        Ok(self
            .client
            .get(GolangEndpoint::VersionsList(package))?
            .text()?
            .split('\n')
            .filter_map(|line| match line {
                "" => None,
                non_empty => Some(non_empty.to_owned()),
            })
            .collect())
    }
}
