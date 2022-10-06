pub mod queries;

use std::collections::HashMap;

use reqwest::{
    blocking::{Client, Response},
    header::{AUTHORIZATION, USER_AGENT},
};
use serde::Serialize;
use serde_json::Value;
use vercel_lambda::error::VercelError;

use super::GitHubRepo;

#[derive(Serialize)]
pub struct GraphQLRequest {
    pub query: String,
    pub variables: HashMap<String, Value>,
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

    fn graphql(&self, request: GraphQLRequest) -> Result<Response, reqwest::Error> {
        self.client
        .post("https://api.github.com/graphql")
        .header(AUTHORIZATION, format!("Bearer {}", self.api_key))
        .header(
            USER_AGENT,
            "vercel-github-api-proxy (+https://github.com/williamboman/vercel-github-api-proxy)",
        )
        .json(&request)
        .send()
    }

    pub fn fetch_latest_tag(&self, repo: &GitHubRepo) -> Result<Response, VercelError> {
        self.graphql(GraphQLRequest {
            query: queries::latest_tag::QUERY.to_owned(),
            variables: HashMap::from([
                ("owner".to_owned(), repo.owner.clone().into()),
                ("name".to_owned(), repo.name.clone().into()),
            ]),
        })
        .map_err(|e| {
            VercelError::new(&format!(
                "Request to {:?} failed with status code {:?}",
                e.url(),
                e.status()
            ))
        })
    }
}
