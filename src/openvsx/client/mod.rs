pub mod spec;

use self::spec::{OpenVSXExtensionVersionsDto, OpenVSXExtensionDto};

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

    pub fn fetch_latest_extension_version(
        &self,
        extension: &OpenVSXExtension,
    ) -> Result<OpenVSXExtensionDto, reqwest::Error> {
        self.client
            .get(OpenVSXEndpoint::Extension(extension))?
            .json()
    }

    pub fn fetch_extension_versions(
        &self,
        extension: &OpenVSXExtension,
    ) -> Result<OpenVSXExtensionVersionsDto, reqwest::Error> {
        self.client
            .get(OpenVSXEndpoint::ExtensionVersions(extension))?
            .json()
    }
}
