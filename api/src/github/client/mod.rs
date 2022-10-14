pub mod graphql;
pub mod response;
pub mod spec;

use std::{collections::VecDeque, convert::TryInto, fmt::Display};

use parse_link_header::Link;
use reqwest::{
    blocking::{Client, Response},
    header::{HeaderMap, ACCEPT, AUTHORIZATION, USER_AGENT},
};
use serde::{de::DeserializeOwned, Serialize};
use vercel_lambda::error::VercelError;

use self::{
    graphql::tags::{Edge, TagNode, TagsQuery},
    response::GitHubResponse,
    spec::GitHubReleaseDto,
};

use super::GitHubRepo;

#[derive(Serialize)]
pub struct GraphQLRequest<Variables: Serialize> {
    pub query: String,
    pub variables: Variables,
}

enum GitHubApiEndpoint<'a> {
    GraphQL,
    Link(Link),
    Releases(&'a GitHubRepo),
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
                f.write_fmt(format_args!("/repos/{}/{}/releases", repo.owner, repo.name))
            }
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
    const DEFAULT_PAGE_LIMIT: u8 = 25;

    pub fn with_page(page: u8) -> Self {
        Self {
            page,
            per_page: Self::DEFAULT_PAGE_LIMIT,
        }
    }

    pub fn with_page_limit(mut self, page_limit: u8) -> Self {
        self.per_page = page_limit;
        self
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

    pub fn paginate<T, Init, Cond>(&self, init: Init, cond: Cond) -> Result<Vec<T>, VercelError>
    where
        T: DeserializeOwned,
        Init: Fn() -> Result<GitHubResponse<Vec<T>>, VercelError>,
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
                    response = self
                        .get(GitHubApiEndpoint::Link(next), None)
                        .map_err(|_| VercelError::new("Failed to paginate."))?
                        .try_into()?;
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        Ok(data)
    }

    pub fn fetch_latest_tag(
        &self,
        repo: &GitHubRepo,
    ) -> Result<GitHubResponse<Edge<TagNode>>, VercelError> {
        let response = self.fetch_tags(&repo, Some(1), None)?;
        let mut tags: VecDeque<Edge<TagNode>> = response.data.tags.into();
        let latest_tag = tags
            .pop_front()
            .ok_or_else(|| VercelError::new("Failed to find tag."))?;
        Ok(GitHubResponse {
            data: latest_tag,
            links: None,
        })
    }

    pub fn fetch_tags(
        &self,
        repo: &GitHubRepo,
        first: Option<u64>,
        after: Option<String>,
    ) -> Result<GitHubResponse<TagsQuery>, VercelError> {
        self.graphql(GraphQLRequest {
            query: graphql::tags::QUERY.to_owned(),
            variables: graphql::tags::Variables {
                owner: repo.owner.clone(),
                name: repo.name.clone(),
                first,
                after,
            },
        })
        .map_err(|e| {
            VercelError::new(&format!(
                "Failed to fetch tags. {:?} {:?}",
                e.url(),
                e.status()
            ))
        })?
        .try_into()
    }

    pub fn fetch_releases(
        &self,
        repo: &GitHubRepo,
        pagination: Option<GitHubPagination>,
    ) -> Result<GitHubResponse<Vec<GitHubReleaseDto>>, VercelError> {
        self.get(GitHubApiEndpoint::Releases(repo), pagination)
            .map_err(|e| {
                VercelError::new(&format!(
                    "Failed to fetch releases. {:?} {:?}",
                    e.url(),
                    e.status()
                ))
            })?
            .try_into()
    }

    pub fn fetch_latest_release(
        &self,
        repo: &GitHubRepo,
        include_prerelease: bool,
    ) -> Result<GitHubReleaseDto, VercelError> {
        let is_latest_release = |release: &GitHubReleaseDto| {
            if include_prerelease {
                !release.draft
            } else {
                !release.draft && !release.prerelease
            }
        };
        let releases = self.paginate(
            || {
                self.fetch_releases(
                    &repo,
                    Some(GitHubPagination {
                        page: 1,
                        per_page: GitHubPagination::MAX_PAGE_LIMIT,
                    }),
                )
            },
            |response| {
                (&response.data)
                    .into_iter()
                    .find(|r| is_latest_release(*r))
                    .is_none()
            },
        )?;
        releases.into_iter().find(is_latest_release).ok_or_else(|| {
            VercelError::new(&format!("Unable to find latest release for repo {}.", repo))
        })
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
