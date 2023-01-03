use std::fmt::Display;

use crate::http::client::{Client, HttpEndpoint};

use self::spec::{RubyGemDto, RubyGemVersionDto};

use super::RubyGemPackage;

pub mod spec;

pub enum RubyGemsEndpoint<'a> {
    Gem(&'a RubyGemPackage),
    GemVersions(&'a RubyGemPackage),
}

impl<'a> HttpEndpoint for RubyGemsEndpoint<'a> {
    fn as_full_url(&self) -> String {
        format!("https://rubygems.org/api/{}", self)
    }
}

impl<'a> Display for RubyGemsEndpoint<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RubyGemsEndpoint::Gem(gem) => f.write_fmt(format_args!("v1/gems/{}.json", gem.name)),
            RubyGemsEndpoint::GemVersions(gem) => {
                f.write_fmt(format_args!("v1/versions/{}.json", gem.name))
            }
        }
    }
}

pub struct RubyGemsClient {
    client: Client,
}

impl RubyGemsClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(None),
        }
    }

    pub fn fetch_gem(&self, gem: &RubyGemPackage) -> Result<RubyGemDto, reqwest::Error> {
        self.client.get(RubyGemsEndpoint::Gem(gem))?.json()
    }

    pub fn fetch_gem_versions(
        &self,
        gem: &RubyGemPackage,
    ) -> Result<Vec<RubyGemVersionDto>, reqwest::Error> {
        self.client.get(RubyGemsEndpoint::GemVersions(gem))?.json()
    }
}
