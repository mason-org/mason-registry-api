pub mod graphql;
pub mod spec;

use std::{collections::HashMap, fmt::Display};

use reqwest::{
    blocking::{Client, Response},
    header::{HeaderMap, AUTHORIZATION, USER_AGENT},
};
use serde::Serialize;
use serde_json::Value;
use vercel_lambda::error::VercelError;

use self::{graphql::latest_tag::LatestTagQueryResponse, spec::GitHubRelease};

use super::GitHubRepo;

#[derive(Serialize)]
pub struct GraphQLRequest {
    pub query: String,
    pub variables: HashMap<String, Value>,
}

enum GitHubApiEndpoint<'a> {
    GraphQL,
    Releases(&'a GitHubRepo),
}

impl<'a> GitHubApiEndpoint<'a> {
    fn as_full_url(&self) -> String {
        format!("https://api.github.com{}", self)
    }
}

impl<'a> Display for GitHubApiEndpoint<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GitHubApiEndpoint::GraphQL => f.write_str("/graphql"),
            GitHubApiEndpoint::Releases(repo) => {
                f.write_fmt(format_args!("/repos/{}/{}/releases", repo.owner, repo.name))
            }
        }
    }
}

pub struct GitHubClient {
    client: Client,
    api_key: String,
}

impl GitHubClient {
    pub fn new(api_key: String) -> Self {
        GitHubClient {
            api_key,
            client: reqwest::blocking::Client::new(),
        }
    }

    fn headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            format!("Bearer {}", self.api_key).parse().unwrap(),
        );
        headers.insert(
            USER_AGENT,
            "mason-registry-api (+https://github.com/williamboman/mason-registry-api)"
                .parse()
                .unwrap(),
        );
        headers
    }

    fn graphql(&self, request: GraphQLRequest) -> Result<Response, reqwest::Error> {
        self.post(GitHubApiEndpoint::GraphQL, &request)
    }

    fn get(&self, endpoint: GitHubApiEndpoint) -> Result<Response, reqwest::Error> {
        self.client
            .get(endpoint.as_full_url())
            .headers(self.headers())
            .send()
    }

    fn post<Json: Serialize>(
        &self,
        endpoint: GitHubApiEndpoint,
        json: &Json,
    ) -> Result<Response, reqwest::Error> {
        self.client
            .post(endpoint.as_full_url())
            .headers(self.headers())
            .json(json)
            .send()
    }

    pub fn fetch_latest_tag(
        &self,
        repo: &GitHubRepo,
    ) -> Result<LatestTagQueryResponse, VercelError> {
        self.graphql(GraphQLRequest {
            query: graphql::latest_tag::QUERY.to_owned(),
            variables: HashMap::from([
                ("owner".to_owned(), repo.owner.clone().into()),
                ("name".to_owned(), repo.name.clone().into()),
            ]),
        })
        .map_err(|e| {
            VercelError::new(&format!(
                "Failed to fetch latest tag. {:?} {:?}",
                e.url(),
                e.status()
            ))
        })?
        .json()
        .map_err(|_| VercelError::new("Failed to parse JSON."))
    }

    pub fn fetch_releases(&self, repo: &GitHubRepo) -> Result<Vec<GitHubRelease>, VercelError> {
        self.get(GitHubApiEndpoint::Releases(repo))
            .map_err(|e| {
                VercelError::new(&format!(
                    "Failed to fetch latest tag. {:?} {:?}",
                    e.url(),
                    e.status()
                ))
            })?
            .json()
            .map_err(|_| VercelError::new("Failed to parse JSON."))
    }

    pub fn fetch_latest_release(
        &self,
        repo: &GitHubRepo,
        include_prerelease: bool,
    ) -> Result<GitHubRelease, VercelError> {
        let releases = self.fetch_releases(&repo)?;
        releases
            .into_iter()
            .find(|release| {
                if include_prerelease {
                    !release.draft
                } else {
                    !release.draft && !release.prerelease
                }
            })
            .ok_or_else(|| {
                VercelError::new(&format!("Unable to find latest release for repo {}.", repo))
            })
    }
}
