use std::collections::HashMap;

use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;

#[derive(Serialize)]
pub struct Variables {
    pub owner: String,
    pub name: String,
    pub first: Option<u64>,
    pub after: Option<String>,
}

pub const QUERY: &str = r#"
    query TagsQuery($owner: String!, $name: String!, $first: Int, $after: String) {
      repository(owner: $owner, name: $name) {
        refs(refPrefix: "refs/tags/", first: $first, after: $after, orderBy: { field: TAG_COMMIT_DATE, direction: DESC }) {
          edges {
            cursor
            node {
              name
            }
          }
        }
      }
    }
"#;

#[derive(Debug, Deserialize)]
pub struct Edge<Node> {
    pub cursor: String,
    pub node: Node,
}

#[derive(Debug, Deserialize)]
pub struct TagNode {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct TagsQuery {
    #[serde(rename(deserialize = "data"), deserialize_with = "tag_deserialize")]
    pub tags: Vec<Edge<TagNode>>,
}

fn get_edges(mut response: HashMap<String, Value>) -> Option<Value> {
    Some(
        response
            .remove("repository")?
            .get_mut("refs")?
            .take()
            .get_mut("edges")?
            .take(),
    )
}

fn tag_deserialize<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Vec<Edge<TagNode>>, D::Error> {
    let response: HashMap<String, Value> = HashMap::deserialize(deserializer)?;
    let edges = get_edges(response)
        .ok_or_else(|| serde::de::Error::missing_field("Unable to find edges."))?;
    serde_json::from_value(edges)
        .map_err(|_| serde::de::Error::custom("Failed to deserialize edges."))
}
