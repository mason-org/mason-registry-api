pub mod graphql;
pub mod response;
pub mod spec;

use std::{convert::TryInto, fmt::Display};

use parse_link_header::Link;
use reqwest::{
    blocking::{Client, Response},
    header::{HeaderMap, ACCEPT, AUTHORIZATION, USER_AGENT},
};
use serde::{de::DeserializeOwned, Serialize};

use self::{
    graphql::tags::TagsQuery,
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

impl<'a> GitHubApiEndpoint<'a> {
    fn as_full_url(&self) -> String {
        match self {
            GitHubApiEndpoint::Link(uri) => uri.raw_uri.to_owned(),
            endpoint => format!("https://api.github.com{}", endpoint),
        }
    }
}

impl<'a> Display for GitHubApiEndpoint<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GitHubApiEndpoint::GraphQL => f.write_str("/graphql"),
            GitHubApiEndpoint::Link(link) => f.write_str(&link.raw_uri),
            GitHubApiEndpoint::Releases(repo) => {
                f.write_fmt(format_args!("/repos/{}/releases", repo))
            }
            GitHubApiEndpoint::ReleaseTag(repo, release_tag) => f.write_fmt(format_args!(
                "/repos/{}/releases/tags/{}",
                repo, release_tag
            )),
            GitHubApiEndpoint::GitRef(repo, git_ref) => f.write_fmt(format_args!(
                "/repos/{}/git/ref/{}",
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
    api_key: String,
}

impl GitHubClient {
    pub fn new(api_key: String) -> Self {
        GitHubClient {
            api_key,
            client: reqwest::blocking::Client::new(),
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
                    response = self.get(GitHubApiEndpoint::Link(next), None)?.try_into()?;
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
        first: Option<u64>,
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

    pub fn fetch_ref<GitRef: GitHubRefId>(
        &self,
        repo: &GitHubRepo,
        ref_id: &GitRef,
    ) -> Result<GitHubResponse<GitHubRef>, reqwest::Error> {
        self.get(GitHubApiEndpoint::GitRef(repo, ref_id), None)?
            .try_into()
    }

    pub fn fetch_releases(
        &self,
        repo: &GitHubRepo,
        pagination: Option<GitHubPagination>,
    ) -> Result<GitHubResponse<Vec<GitHubReleaseDto>>, reqwest::Error> {
        self.get(GitHubApiEndpoint::Releases(repo), pagination)?
            .try_into()
    }

    pub fn fetch_release_by_tag(
        &self,
        repo: &GitHubRepo,
        release: &GitHubTag,
    ) -> Result<GitHubResponse<GitHubReleaseDto>, reqwest::Error> {
        self.get(GitHubApiEndpoint::ReleaseTag(&repo, &release), None)?
            .try_into()
    }

    fn headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(
            ACCEPT,
            "application/vnd.github.v3+json; q=1.0, application/json; q=0.8"
                .parse()
                .unwrap(),
        );
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

    fn graphql<Variables: Serialize>(
        &self,
        request: GraphQLRequest<Variables>,
    ) -> Result<Response, reqwest::Error> {
        self.post(GitHubApiEndpoint::GraphQL, &request)
    }

    fn get(
        &self,
        endpoint: GitHubApiEndpoint,
        pagination: Option<GitHubPagination>,
    ) -> Result<Response, reqwest::Error> {
        let query = match pagination {
            Some(pagination) => vec![("page", pagination.page), ("per_page", pagination.per_page)],

            None => vec![],
        };
        self.client
            .get(endpoint.as_full_url())
            .query(&query)
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
}
