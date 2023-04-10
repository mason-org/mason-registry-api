use std::fmt::Display;

pub mod spec;

use http::{header::AUTHORIZATION, HeaderMap};

use crate::{
    github::GitHubRepo,
    http::client::{Client, HttpEndpoint},
};

use self::spec::JobsResponse;

pub struct RenovateClient {
    client: Client,
}

enum RenovateEndpoint<'a> {
    GitHubJobs(&'a GitHubRepo),
}

impl<'a> Display for RenovateEndpoint<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RenovateEndpoint::GitHubJobs(repo) => {
                f.write_fmt(format_args!("github/repos/{}/jobs", repo))
            }
        }
    }
}

impl<'a> HttpEndpoint for RenovateEndpoint<'a> {
    fn as_full_url(&self) -> String {
        format!("https://v1.renovateapi.com/{}", self)
    }
}

impl RenovateClient {
    pub fn new(api_key: String) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            format!("Bearer {}", api_key).parse().unwrap(),
        );
        Self {
            client: Client::new(Some(headers)),
        }
    }

    /// Returns jobs in ASCENDING order.
    pub fn fetch_github_jobs(&self, repo: &GitHubRepo) -> Result<JobsResponse, reqwest::Error> {
        tracing::debug!("Fetching GitHub jobs for repo: {repo}");
        self.client.get(RenovateEndpoint::GitHubJobs(repo))?.json()
    }
}
