use std::fmt::Display;

use reqwest::{
    blocking::{Client, Response},
    header::{HeaderMap, ACCEPT, USER_AGENT},
};
use serde::Serialize;

use self::spec::{RubyGemDto, RubyGemVersionDto};

use super::RubyGemPackage;

pub mod spec;

pub enum RubyGemsEndpoint<'a> {
    Gem(&'a RubyGemPackage),
    GemVersions(&'a RubyGemPackage),
}

impl<'a> RubyGemsEndpoint<'a> {
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

    fn get(&self, endpoint: RubyGemsEndpoint) -> Result<Response, reqwest::Error> {
        self.client
            .get(endpoint.as_full_url())
            .headers(self.headers())
            .send()?
            .error_for_status()
    }

    #[allow(dead_code)]
    fn post<Json: Serialize>(
        &self,
        endpoint: RubyGemsEndpoint,
        json: &Json,
    ) -> Result<Response, reqwest::Error> {
        self.client
            .post(endpoint.as_full_url())
            .headers(self.headers())
            .json(json)
            .send()?
            .error_for_status()
    }

    pub fn fetch_gem(&self, gem: &RubyGemPackage) -> Result<RubyGemDto, reqwest::Error> {
        self.get(RubyGemsEndpoint::Gem(gem))?.json()
    }

    pub fn fetch_gem_versions(
        &self,
        gem: &RubyGemPackage,
    ) -> Result<Vec<RubyGemVersionDto>, reqwest::Error> {
        self.get(RubyGemsEndpoint::GemVersions(gem))?.json()
    }
}
