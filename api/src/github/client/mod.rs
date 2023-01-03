pub mod graphql;
pub mod response;
pub mod spec;

use std::{convert::TryInto, fmt::Display};

use parse_link_header::Link;
use reqwest::{
    blocking::Response,
    header::{HeaderMap, ACCEPT, AUTHORIZATION},
};
use serde::{de::DeserializeOwned, Serialize};

use crate::http::client::{Client, HttpEndpoint};

use self::{
    graphql::{sponsors::SponsorsQuery, tags::TagsQuery},
    response::GitHubResponse,
    spec::{GitHubRef, GitHubReleaseDto},
};

use super::{GitHubRefId, GitHubRepo, GitHubTag};

#[derive(Serialize)]
pub struct GraphQLRequest<Variables: Serialize> {
    pub query: String,
    pub variables: Variables,
}

enum GitHubApiEndpoint<'a> {
    GraphQL,
    Link(Link),
    Releases(&'a GitHubRepo),
    ReleaseTag(&'a GitHubRepo, &'a GitHubTag),
    GitRef(&'a GitHubRepo, &'a dyn GitHubRefId),
}

impl<'a> HttpEndpoint for GitHubApiEndpoint<'a> {
    fn as_full_url(&self) -> String {
        match self {
            GitHubApiEndpoint::Link(uri) => uri.raw_uri.to_owned(),
            endpoint => format!("https://api.github.com/{}", endpoint),
        }
    }
}

impl<'a> Display for GitHubApiEndpoint<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GitHubApiEndpoint::GraphQL => f.write_str("graphql"),
            GitHubApiEndpoint::Link(link) => f.write_str(&link.raw_uri),
            GitHubApiEndpoint::Releases(repo) => {
                f.write_fmt(format_args!("repos/{}/releases", repo))
            }
            GitHubApiEndpoint::ReleaseTag(repo, release_tag) => f.write_fmt(format_args!(
                "repos/{}/releases/tags/{}",
                repo, release_tag
            )),
            GitHubApiEndpoint::GitRef(repo, git_ref) => f.write_fmt(format_args!(
                "repos/{}/git/ref/{}",
                repo,
                git_ref.get_ref_endpoint()
            )),
        }
    }
}

#[derive(Debug)]
pub struct GitHubPagination {
    pub page: u8,
    pub per_page: u8,
}

impl GitHubPagination {
    pub const MAX_PAGE_LIMIT: u8 = 100;
}

pub struct GitHubClient {
    client: Client,
}

impl GitHubClient {
    pub fn new(api_key: String) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(
            ACCEPT,
            "application/vnd.github.v3+json; q=1.0, application/json; q=0.8"
                .parse()
                .unwrap(),
        );
        headers.insert(
            AUTHORIZATION,
            format!("Bearer {}", api_key).parse().unwrap(),
        );
        GitHubClient {
            client: Client::new(Some(headers)),
        }
    }

    pub fn paginate<T, Init, Cond>(&self, init: Init, cond: Cond) -> Result<Vec<T>, reqwest::Error>
    where
        T: DeserializeOwned,
        Init: Fn() -> Result<GitHubResponse<Vec<T>>, reqwest::Error>,
        Cond: Fn(&GitHubResponse<Vec<T>>) -> bool,
    {
        let mut data = Vec::with_capacity(GitHubPagination::MAX_PAGE_LIMIT.into());
        let mut response = init()?;
        loop {
            let should_continue = cond(&response);
            data.append(&mut response.data);
            if !should_continue {
                break;
            }
            if let Some(mut links) = response.links {
                if let Some(next) = links.remove(&Some("next".to_owned())) {
                    response = self.client.get(GitHubApiEndpoint::Link(next))?.try_into()?;
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        Ok(data)
    }

    pub fn fetch_tags(
        &self,
        repo: &GitHubRepo,
        first: u64,
        after: Option<String>,
    ) -> Result<GitHubResponse<TagsQuery>, reqwest::Error> {
        self.graphql(GraphQLRequest {
            query: graphql::tags::QUERY.to_owned(),
            variables: graphql::tags::Variables {
                owner: repo.owner.clone(),
                name: repo.name.clone(),
                first,
                after,
            },
        })?
        .try_into()
    }

    pub fn fetch_sponsors(
        &self,
        login: String,
        first: u64,
        after: Option<String>,
    ) -> Result<GitHubResponse<SponsorsQuery>, reqwest::Error> {
        self.graphql(GraphQLRequest {
            query: graphql::sponsors::QUERY.to_owned(),
            variables: graphql::sponsors::Variables {
                login,
                first,
                after,
            },
        })?
        .try_into()
    }

    pub fn fetch_ref<GitRef: GitHubRefId>(
        &self,
        repo: &GitHubRepo,
        ref_id: &GitRef,
    ) -> Result<GitHubResponse<GitHubRef>, reqwest::Error> {
        self.client
            .get(GitHubApiEndpoint::GitRef(repo, ref_id))?
            .try_into()
    }

    pub fn fetch_releases(
        &self,
        repo: &GitHubRepo,
        pagination: Option<GitHubPagination>,
    ) -> Result<GitHubResponse<Vec<GitHubReleaseDto>>, reqwest::Error> {
        match pagination {
            Some(pagination) => {
                self.get_with_pagination(GitHubApiEndpoint::Releases(repo), pagination)
            }
            None => self.client.get(GitHubApiEndpoint::Releases(repo)),
        }?
        .try_into()
    }

    pub fn fetch_release_by_tag(
        &self,
        repo: &GitHubRepo,
        release: &GitHubTag,
    ) -> Result<GitHubResponse<GitHubReleaseDto>, reqwest::Error> {
        self.client
            .get(GitHubApiEndpoint::ReleaseTag(&repo, &release))?
            .try_into()
    }

    fn graphql<Variables: Serialize>(
        &self,
        request: GraphQLRequest<Variables>,
    ) -> Result<Response, reqwest::Error> {
        self.client.post(GitHubApiEndpoint::GraphQL, &request)
    }

    fn get_with_pagination(
        &self,
        endpoint: GitHubApiEndpoint,
        pagination: GitHubPagination,
    ) -> Result<Response, reqwest::Error> {
        let query = vec![("page", pagination.page), ("per_page", pagination.per_page)];
        self.client.get_with_query(endpoint, &query)
    }
}
