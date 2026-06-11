pub mod spec;

use self::spec::{OpenVSXExtensionDto, OpenVSXExtensionVersionsDto};

use super::OpenVSXExtension;
use crate::http::client::{Client, HttpEndpoint};
use std::fmt::Display;

pub struct OpenVSXClient {
    client: Client,
}

enum OpenVSXEndpoint<'a> {
    Extension(&'a OpenVSXExtension),
    ExtensionVersions(&'a OpenVSXExtension),
}

impl<'a> HttpEndpoint for OpenVSXEndpoint<'a> {
    fn as_full_url(&self) -> String {
        format!("https://open-vsx.org/api/{}", self)
    }
}

impl<'a> Display for OpenVSXEndpoint<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OpenVSXEndpoint::Extension(ext) => {
                f.write_fmt(format_args!("{}/{}", ext.namespace, ext.extension))
            }
            OpenVSXEndpoint::ExtensionVersions(ext) => {
                f.write_fmt(format_args!("{}/{}/versions", ext.namespace, ext.extension))
            }
        }
    }
}

impl OpenVSXClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(None),
        }
    }

    pub async fn fetch_extension(
        &self,
        extension: &OpenVSXExtension,
    ) -> Result<OpenVSXExtensionDto, reqwest::Error> {
        self.client
            .get(OpenVSXEndpoint::Extension(extension))
            .await?
            .json()
            .await
    }

    pub async fn fetch_extension_versions(
        &self,
        extension: &OpenVSXExtension,
        size: u64,
        offset: u64,
    ) -> Result<OpenVSXExtensionVersionsDto, reqwest::Error> {
        let query = vec![("size", size), ("offset", offset)];
        self.client
            .get_with_query(OpenVSXEndpoint::ExtensionVersions(extension), &query)?
            .await?
            .json()
            .await
    }
}
