use std::{collections::HashMap, ops::Deref};

use serde::Serialize;
use tracing_subscriber::FmtSubscriber;

pub mod badges;
pub mod crates;
pub mod errors;
pub mod github;
pub mod golang;
pub mod http;
pub mod npm;
pub mod openvsx;
pub mod packagist;
pub mod pypi;
pub mod renovate;
pub mod rubygems;
pub mod vercel;

pub struct QueryParams(HashMap<String, String>);

impl QueryParams {
    pub fn get(&self, query: &str) -> Option<&String> {
        self.0.get(query)
    }

    pub fn has_flag(&self, query: &str) -> bool {
        match self.0.get(query).map(Deref::deref) {
            Some("") | Some("1") | Some("true") => return true,
            _ => return false,
        }
    }
}

impl From<&url::Url> for QueryParams {
    fn from(url: &url::Url) -> Self {
        let mut query = HashMap::new();
        for (key, val) in url.query_pairs().into_owned() {
            query.insert(key, val);
        }
        QueryParams(query)
    }
}

pub enum CacheControl {
    NoStore,
    PublicShort,
    PublicMedium,
    PublicLong,
}

impl CacheControl {
    pub fn get_header(&self) -> &'static str {
        match self {
            CacheControl::NoStore => "no-store",
            CacheControl::PublicShort => "max-age=0, s-maxage=60, stale-while-revalidate=120",
            CacheControl::PublicMedium => "max-age=0, s-maxage=900",
            CacheControl::PublicLong => "max-age=0, s-maxage=86400",
        }
    }
}

#[derive(Serialize)]
struct ErrResponse {
    message: String,
}

pub fn setup_tracing() {
    let tracing_level: Option<&'static str> = std::option_env!("TRACING_LEVEL");
    let level = tracing_level
        .and_then(|level| level.parse().ok())
        .unwrap_or(tracing::Level::INFO);

    let subscriber = FmtSubscriber::builder().with_max_level(level).finish();

    let _ = tracing::subscriber::set_global_default(subscriber);
}

#[cfg(test)]
mod tests {
    use super::*;
    use url::Url;

    #[test]
    fn should_parse_query_flags() {
        let query: QueryParams = (&Url::parse(
            "https://api.mason-registry.dev/api/endpoint?do_something=1&do_something_else=true&do&not=false",
        )
        .unwrap()).into();

        assert!(query.has_flag("do_something"));
        assert!(query.has_flag("do_something_else"));
        assert!(query.has_flag("do"));
        assert!(!query.has_flag("do_nothing"));
        assert!(!query.has_flag("not"));
    }
}
