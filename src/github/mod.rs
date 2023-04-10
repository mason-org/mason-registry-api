pub mod api;
pub mod client;
pub mod errors;
pub mod manager;

use std::{fmt::Display, str::FromStr};

use crate::QueryParams;

#[derive(Debug)]
pub struct GitHubRepo {
    pub owner: String,
    pub name: String,
}

impl GitHubRepo {
    pub fn new(owner: String, name: String) -> Self {
        Self { owner, name }
    }
}

impl From<GitHubRepo> for crate::CacheControl {
    fn from(repo: GitHubRepo) -> Self {
        match repo {
            GitHubRepo {
                ref owner,
                ref name,
            } if owner == "mason-org" && name == "mason-registry" => Self::PublicShort,
            _ => Self::PublicMedium,
        }
    }
}

impl Display for GitHubRepo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.owner, self.name)
    }
}

impl From<&QueryParams> for GitHubRepo {
    fn from(query: &QueryParams) -> Self {
        match (query.get("owner"), query.get("name")) {
            (Some(owner), Some(name)) => Self {
                owner: owner.to_owned(),
                name: name.to_owned(),
            },
            (Some(_), None) | (None, None) | (None, Some(_)) => {
                panic!("Failed to parse GitHub repo from URL.")
            }
        }
    }
}

pub trait GitHubRefId {
    fn get_ref_endpoint(&self) -> String;
}

pub struct GitHubTag(String);

impl GitHubRefId for GitHubTag {
    fn get_ref_endpoint(&self) -> String {
        format!("tags/{}", self.0)
    }
}

impl Display for GitHubTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl FromStr for GitHubTag {
    type Err = Box<dyn std::error::Error + Send + Sync + 'static>;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        Ok(Self(str.to_owned()))
    }
}
