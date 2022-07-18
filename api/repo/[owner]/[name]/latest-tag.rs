use core::str;
use http::{
    header::{CACHE_CONTROL, CONTENT_TYPE},
    Method, StatusCode,
};
use reqwest::header::{AUTHORIZATION, USER_AGENT};
use serde::{de::Unexpected, Deserialize, Serialize};
use std::{collections::HashMap, error::Error, fmt::Display, str::FromStr};
use vercel_lambda::{error::VercelError, lambda, Body, IntoResponse, Request, Response};

#[derive(Serialize)]
struct GraphQLRequest {
    query: String,
    variables: HashMap<String, String>,
}

static LATEST_TAG_QUERY: &str = r#"
    query ($owner: String!, $name: String!) {
      repository(owner: $owner, name: $name) {
        refs(refPrefix: "refs/tags/", first: 1, orderBy: {field: TAG_COMMIT_DATE, direction: DESC}) {
          edges {
            node {
              name
              target {
                oid
                ... on Tag {
                  message
                  commitUrl
                  tagger {
                    name
                    email
                    date
                  }
                }
              }
            }
          }
        }
      }
    }
"#;

#[derive(Serialize)]
struct LatestTagQueryResponse {
    tag: String,
}

impl LatestTagQueryResponse {
    fn get_node_from_original_response(
        response: &HashMap<String, serde_json::Value>,
    ) -> Option<&serde_json::Value> {
        Some(
            response
                .get("data")?
                .get("repository")?
                .get("refs")?
                .get("edges")?
                .get(0)?
                .get("node")?,
        )
    }
}

impl<'de> Deserialize<'de> for LatestTagQueryResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let response: HashMap<String, serde_json::Value> = HashMap::deserialize(deserializer)?;
        let node = Self::get_node_from_original_response(&response)
            .ok_or_else(|| serde::de::Error::missing_field("Failed to find node field."))?;

        let name_value = node
            .get("name")
            .ok_or_else(|| serde::de::Error::missing_field("name"))?;

        match name_value {
            serde_json::Value::String(tag) => Ok(LatestTagQueryResponse {
                tag: tag.to_owned(),
            }),
            _ => Err(serde::de::Error::invalid_type(
                // TODO: would be nice to be able to convert a `Value` to an `Unexpected` ¯\_(ツ)_/¯
                Unexpected::Other(&""),
                &"a string",
            )),
        }
    }
}

#[derive(Debug)]
struct Repo {
    owner: String,
    name: String,
}

impl Display for Repo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.owner, self.name)
    }
}

impl From<UriQueryParams> for Repo {
    fn from(params: UriQueryParams) -> Self {
        if let (Some(owner), Some(name)) = (
            params.params.get("owner").and_then(|o| o.to_owned()),
            params.params.get("name").and_then(|o| o.to_owned()),
        ) {
            return Self { owner, name };
        }
        panic!("Failed to parse repo from {:?}", params)
    }
}

#[derive(Debug)]
struct UriQueryParams {
    params: HashMap<String, Option<String>>,
}

impl FromStr for UriQueryParams {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut params: HashMap<String, Option<String>> = HashMap::new();
        for part in s.split("&") {
            let mut param = part.splitn(2, "=");
            let key = param.next().expect("Failed to get UriQueryParams key.");
            let value = param.next();
            params.insert(key.to_owned(), value.map(ToOwned::to_owned));
        }
        return Ok(UriQueryParams { params });
    }
}

fn handler(request: Request) -> Result<impl IntoResponse, VercelError> {
    let api_key: String =
        std::env::var("GITHUB_API_KEY").expect("No GITHUB_API_KEY environment variable.");

    let query_params: UriQueryParams = request
        .uri()
        .query()
        .expect("No query parameters.")
        .parse()
        .unwrap_or_else(|_| panic!("Failed to parse query parameters."));

    if request.method() != Method::GET {
        return Ok(Response::builder()
            .status(StatusCode::METHOD_NOT_ALLOWED)
            .body(Body::Empty)
            .expect(""));
    }

    let repo: Repo = query_params.into();

    let request = GraphQLRequest {
        query: LATEST_TAG_QUERY.to_owned(),
        variables: HashMap::from([
            ("owner".to_owned(), repo.owner),
            ("name".to_owned(), repo.name),
        ]),
    };

    let client = reqwest::blocking::Client::new();

    let graphql_response = client
        .post("https://api.github.com/graphql")
        .header(AUTHORIZATION, format!("Bearer {}", api_key))
        .header(
            USER_AGENT,
            "vercel-github-api-latest-tag-proxy (+https://github.com/williamboman/vercel-github-api-latest-tag-proxy)",
        )
        .json(&request)
        .send()
        .expect("GraphQL request failed.");

    let json: LatestTagQueryResponse = graphql_response
        .json()
        .expect("Failed to parse GraphQL response.");

    let response = Response::builder()
        .status(200)
        .header(CONTENT_TYPE, "application/json")
        .header(CACHE_CONTROL, "public, s-maxage=1800")
        .body(Body::Text(
            serde_json::to_string_pretty(&json).expect("Failed to serialize JSON."),
        ))
        .expect("Internal Server Error");

    Ok(response)
}

// Start the runtime with the handler
fn main() -> Result<(), Box<dyn Error>> {
    Ok(lambda!(handler))
}
