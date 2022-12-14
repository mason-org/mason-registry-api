use std::collections::HashMap;

use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;

use super::PageInfo;

#[derive(Serialize)]
pub struct Variables {
    pub login: String,
    pub first: u64,
    pub after: Option<String>,
}

pub const QUERY: &str = r#"
    query SponsorsQuery($login: String!, $first: Int!, $after: String) {
      user(login: $login) {
        sponsors(first: $first, after: $after) {
          pageInfo {
              startCursor
              endCursor
              hasNextPage
              hasPreviousPage
          }
          nodes {
            ... on Actor {
              login
              avatarUrl
              url
            }
          }
        }
      }
    }
"#;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sponsor {
    pub avatar_url: String,
    pub login: String,
    pub url: String,
}

#[derive(Debug)]
pub struct SponsorsQuery {
    pub sponsors: Vec<Sponsor>,
    pub page_info: PageInfo,
}

impl<'de> Deserialize<'de> for SponsorsQuery {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut response: HashMap<String, Value> = HashMap::deserialize(deserializer)?;
        let mut sponsors_connection = response
            .remove("data")
            .ok_or_else(|| serde::de::Error::missing_field("data key missing."))?
            .get_mut("user")
            .ok_or_else(|| serde::de::Error::missing_field("user key missing."))?
            .take()
            .get_mut("sponsors")
            .ok_or_else(|| serde::de::Error::missing_field("user.sponsors key missing."))?
            .take();

        let sponsors = sponsors_connection
            .get_mut("nodes")
            .ok_or_else(|| serde::de::Error::missing_field("sponsors key missing."))?
            .take();

        let page_info = sponsors_connection
            .get_mut("pageInfo")
            .ok_or_else(|| serde::de::Error::missing_field("pageInfo key missing."))?
            .take();

        Ok(Self {
            sponsors: serde_json::from_value(sponsors)
                .map_err(|_| serde::de::Error::custom("Failed to deserialize sponsors."))?,
            page_info: serde_json::from_value(page_info)
                .map_err(|_| serde::de::Error::custom("Failed to deserialize pageInfo."))?,
        })
    }
}
