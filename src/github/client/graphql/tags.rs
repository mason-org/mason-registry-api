use std::collections::HashMap;

use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;

use super::PageInfo;

#[derive(Serialize)]
pub struct Variables {
    pub owner: String,
    pub name: String,
    pub first: u64,
    pub after: Option<String>,
}

pub const QUERY: &str = r#"
    query TagsQuery($owner: String!, $name: String!, $first: Int!, $after: String) {
      repository(owner: $owner, name: $name) {
        refs(refPrefix: "refs/tags/", first: $first, after: $after, orderBy: { field: TAG_COMMIT_DATE, direction: DESC }) {
          pageInfo {
              startCursor
              endCursor
              hasNextPage
              hasPreviousPage
          }
          nodes {
            name
          }
        }
      }
    }
"#;

#[derive(Debug, Deserialize)]
pub struct Tag {
    pub name: String,
}

#[derive(Debug)]
pub struct TagsQuery {
    pub tags: Vec<Tag>,
    pub page_info: PageInfo,
}

impl<'de> Deserialize<'de> for TagsQuery {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut response: HashMap<String, Value> = HashMap::deserialize(deserializer)?;
        let mut tags_connection = response
            .remove("data")
            .ok_or_else(|| serde::de::Error::missing_field("data key missing."))?
            .get_mut("repository")
            .ok_or_else(|| serde::de::Error::missing_field("repository key missing."))?
            .take()
            .get_mut("refs")
            .ok_or_else(|| serde::de::Error::missing_field("repository.refs key missing."))?
            .take();

        let tags = tags_connection
            .get_mut("nodes")
            .ok_or_else(|| serde::de::Error::missing_field("nodes key missing."))?
            .take();

        let page_info = tags_connection
            .get_mut("pageInfo")
            .ok_or_else(|| serde::de::Error::missing_field("pageInfo key missing."))?
            .take();

        Ok(Self {
            tags: serde_json::from_value(tags)
                .map_err(|_| serde::de::Error::custom("Failed to deserialize tags."))?,
            page_info: serde_json::from_value(page_info)
                .map_err(|_| serde::de::Error::custom("Failed to deserialize pageInfo."))?,
        })
    }
}
