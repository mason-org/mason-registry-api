use std::{collections::HashMap, ops::Deref};

use ::http::{
    header::{CACHE_CONTROL, CONTENT_TYPE},
    Response, StatusCode,
};
use errors::ApiError;
use serde::Serialize;
use vercel_lambda::{error::VercelError, Body};

pub mod crates;
pub mod errors;
pub mod github;
pub mod golang;
pub mod http;
pub mod npm;
pub mod packagist;
pub mod pypi;
pub mod rubygems;

pub fn parse_url(request: &vercel_lambda::Request) -> Result<url::Url, VercelError> {
    url::Url::parse(&request.uri().to_string())
        .map_err(|_| VercelError::new("Failed to parse request URI."))
}

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
}

fn json_response<T: Serialize>(
    status: StatusCode,
    cache: CacheControl,
    data: &T,
) -> Result<Response<Body>, VercelError> {
    Response::builder()
        .status(status)
        .header(CONTENT_TYPE, "application/json")
        .header(
            CACHE_CONTROL,
            match cache {
                CacheControl::NoStore => "no-store",
                CacheControl::PublicShort => "s-maxage=60, stale-while-revalidate=120",
                CacheControl::PublicMedium => "s-maxage=1800",
            },
        )
        .body(Body::Text(
            serde_json::to_string_pretty(data)
                .map_err(|_| VercelError::new("Failed to serialize."))?,
        ))
        .map_err(|_| VercelError::new("Failed to build response."))
}

pub fn ok_json<T: Serialize>(data: T, cache: CacheControl) -> Result<Response<Body>, VercelError> {
    json_response(StatusCode::OK, cache, &data)
}

#[derive(Serialize)]
struct ErrResponse {
    message: String,
}

pub fn err_json<T: ApiError>(err: T) -> Result<Response<Body>, VercelError> {
    eprintln!("{}", err);
    json_response(
        err.status_code(),
        CacheControl::NoStore,
        &ErrResponse {
            message: err.to_string(),
        },
    )
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
