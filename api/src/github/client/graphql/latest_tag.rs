use std::collections::HashMap;

use serde::{de::Unexpected, Deserialize, Serialize};

pub(crate) static QUERY: &str = r#"
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
pub struct LatestTagQuery {
    pub tag: String,
}

impl LatestTagQuery {
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

impl<'de> Deserialize<'de> for LatestTagQuery {
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
            serde_json::Value::String(tag) => Ok(LatestTagQuery {
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
