pub mod graphql;
pub mod response;
pub mod spec;

use std::fmt::Display;

use parse_link_header::Link;
use reqwest::{
    Response,
    header::{ACCEPT, AUTHORIZATION, HeaderMap},
};
use serde::{Serialize, de::DeserializeOwned};

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
    LatestRelease(&'a GitHubRepo),
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
            GitHubApiEndpoint::LatestRelease(repo) => {
                f.write_fmt(format_args!("repos/{}/releases/latest", repo))
            }
            GitHubApiEndpoint::ReleaseTag(repo, release_tag) => {
                f.write_fmt(format_args!("repos/{}/releases/tags/{}", repo, release_tag))
            }
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

    pub async fn paginate<T, Cond>(
        &self,
        mut cursor: GitHubResponse<Vec<T>>,
        cond: Cond,
    ) -> Result<Vec<T>, reqwest::Error>
    where
        T: DeserializeOwned,
        Cond: Fn(&GitHubResponse<Vec<T>>) -> bool,
    {
        let mut data = Vec::with_capacity(GitHubPagination::MAX_PAGE_LIMIT.into());
        loop {
            let should_continue = cond(&cursor);
            data.append(&mut cursor.data);
            if !should_continue {
                break;
            }
            if let Some(mut links) = cursor.links {
                if let Some(next) = links.remove(&Some("next".to_owned())) {
                    cursor = GitHubResponse::from_response(
                        self.client.get(GitHubApiEndpoint::Link(next)).await?,
                    )
                    .await?
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        Ok(data)
    }

    pub async fn fetch_tags(
        &self,
        repo: &GitHubRepo,
        first: u64,
        after: Option<String>,
    ) -> Result<GitHubResponse<TagsQuery>, reqwest::Error> {
        GitHubResponse::from_response(
            self.graphql(GraphQLRequest {
                query: graphql::tags::QUERY.to_owned(),
                variables: graphql::tags::Variables {
                    owner: repo.owner.clone(),
                    name: repo.name.clone(),
                    first,
                    after,
                },
            })
            .await?,
        )
        .await
    }

    pub async fn fetch_sponsors(
        &self,
        login: String,
        first: u64,
        after: Option<String>,
    ) -> Result<GitHubResponse<SponsorsQuery>, reqwest::Error> {
        GitHubResponse::from_response(
            self.graphql(GraphQLRequest {
                query: graphql::sponsors::QUERY.to_owned(),
                variables: graphql::sponsors::Variables {
                    login,
                    first,
                    after,
                },
            })
            .await?,
        )
        .await
    }

    pub async fn fetch_ref<GitRef: GitHubRefId>(
        &self,
        repo: &GitHubRepo,
        ref_id: &GitRef,
    ) -> Result<GitHubResponse<GitHubRef>, reqwest::Error> {
        GitHubResponse::from_response(
            self.client
                .get(GitHubApiEndpoint::GitRef(repo, ref_id))
                .await?,
        )
        .await
    }

    pub async fn fetch_releases(
        &self,
        repo: &GitHubRepo,
        pagination: Option<GitHubPagination>,
    ) -> Result<GitHubResponse<Vec<GitHubReleaseDto>>, reqwest::Error> {
        GitHubResponse::from_response(match pagination {
            Some(pagination) => {
                self.get_with_pagination(GitHubApiEndpoint::Releases(repo), pagination)
                    .await?
            }
            None => self.client.get(GitHubApiEndpoint::Releases(repo)).await?,
        })
        .await
    }

    pub async fn fetch_release_by_tag(
        &self,
        repo: &GitHubRepo,
        release: &GitHubTag,
    ) -> Result<GitHubResponse<GitHubReleaseDto>, reqwest::Error> {
        GitHubResponse::from_response(
            self.client
                .get(GitHubApiEndpoint::ReleaseTag(repo, release))
                .await?,
        )
        .await
    }

    pub async fn fetch_latest_release(
        &self,
        repo: &GitHubRepo,
    ) -> Result<GitHubResponse<GitHubReleaseDto>, reqwest::Error> {
        GitHubResponse::from_response(
            self.client
                .get(GitHubApiEndpoint::LatestRelease(repo))
                .await?,
        )
        .await
    }

    async fn graphql<Variables: Serialize>(
        &self,
        request: GraphQLRequest<Variables>,
    ) -> Result<Response, reqwest::Error> {
        self.client.post(GitHubApiEndpoint::GraphQL, &request).await
    }

    async fn get_with_pagination<'a>(
        &self,
        endpoint: GitHubApiEndpoint<'a>,
        pagination: GitHubPagination,
    ) -> Result<Response, reqwest::Error> {
        let query = vec![("page", pagination.page), ("per_page", pagination.per_page)];
        self.client.get_with_query(endpoint, &query).await
    }
}
