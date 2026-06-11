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

#[derive(Debug)]
pub struct QueryParams(HashMap<String, String>);

impl QueryParams {
    pub fn get(&self, query: &str) -> Option<&String> {
        self.0.get(query)
    }

    pub fn has_flag(&self, query: &str) -> bool {
        matches!(
            self.0.get(query).map(Deref::deref),
            Some("") | Some("1") | Some("true")
        )
    }
}

impl From<&str> for QueryParams {
    fn from(s: &str) -> Self {
        QueryParams(
            url::form_urlencoded::parse(s.as_bytes())
                .into_owned()
                .collect(),
        )
    }
}

impl From<&vercel_runtime::Request> for QueryParams {
    fn from(request: &vercel_runtime::Request) -> Self {
        request.uri().query().unwrap_or_default().into()
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

    #[test]
    fn should_parse_query_flags() {
        let query: QueryParams = "do_something=1&do_something_else=true&do&not=false".into();

        assert!(query.has_flag("do_something"));
        assert!(query.has_flag("do_something_else"));
        assert!(query.has_flag("do"));
        assert!(!query.has_flag("do_nothing"));
        assert!(!query.has_flag("not"));
    }
}
