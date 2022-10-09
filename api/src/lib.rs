use std::{borrow::Cow, ops::Deref};

use vercel_lambda::error::VercelError;

pub mod api;
pub mod github;
pub mod npm;

pub fn parse_url(request: &vercel_lambda::Request) -> Result<url::Url, VercelError> {
    url::Url::parse(&request.uri().to_string())
        .map_err(|_| VercelError::new("Failed to parse request URI."))
}

pub fn url_has_query_flag(url: &url::Url, query: &str) -> bool {
    for (key, val) in url.query_pairs() {
        if key == Cow::Borrowed(query) {
            match val.deref() {
                "" | "1" | "true" => return true,
                _ => return false,
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use url::Url;

    #[test]
    fn should_parse_query_flags() {
        let url = Url::parse(
            "https://api.mason-registry.dev/api/endpoint?do_something=1&do_something_else=true&do&not=false",
        )
        .unwrap();

        assert!(url_has_query_flag(&url, "do_something"));
        assert!(url_has_query_flag(&url, "do_something_else"));
        assert!(url_has_query_flag(&url, "do"));
        assert!(!url_has_query_flag(&url, "do_nothing"));
        assert!(!url_has_query_flag(&url, "not"));
    }
}
