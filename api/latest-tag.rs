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

impl FromStr for Repo {
    type Err = VercelError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((owner, name)) = s.split_once('/') {
            Ok(Self {
                owner: owner.to_owned(),
                name: name.to_owned(),
            })
        } else {
            Err(VercelError::new(
                format!("Failed to parse repository {}.", s).as_str(),
            ))
        }
    }
}

fn handler(request: Request) -> Result<impl IntoResponse, VercelError> {
    let api_key: String =
        std::env::var("GITHUB_API_KEY").expect("No GITHUB_API_KEY environment variable.");

    if request.method() != Method::GET {
        return Ok(Response::builder()
            .status(StatusCode::METHOD_NOT_ALLOWED)
            .body(Body::Empty)
            .expect(""));
    }

    let params: HashMap<String, String> = request
        .uri()
        .query()
        .map(|v| {
            url::form_urlencoded::parse(v.as_bytes())
                .into_owned()
                .collect()
        })
        .unwrap_or_else(HashMap::new);

    let repo: Repo = params
        .get("repo")
        .ok_or_else(|| VercelError::new("No repo provided."))?
        .parse()
        .expect("Failed to parse repo.");

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
