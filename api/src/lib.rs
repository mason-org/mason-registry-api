use std::{collections::HashMap, str::FromStr};

use vercel_lambda::error::VercelError;

pub mod github;
pub mod api;

#[derive(Debug)]
pub struct UriQueryParams {
    params: HashMap<String, Option<String>>,
}

impl UriQueryParams {
    pub fn get<T>(&self, key: T) -> &Option<String>
    where
        T: AsRef<str>,
    {
        self.params.get(key.as_ref()).unwrap_or(&None)
    }

    pub fn has_flag<T>(&self, key: T) -> bool
    where
        T: AsRef<str>,
    {
        match self.get(key).as_deref() {
            Some("1") | Some("true") => true,
            _ => false,
        }
    }
}

impl FromStr for UriQueryParams {
    type Err = VercelError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut params: HashMap<String, Option<String>> = HashMap::new();
        for part in s.split("&") {
            let mut param = part.splitn(2, "=");
            let key = param
                .next()
                .ok_or_else(|| VercelError::new("Failed to get UriQueryParams key."))?;
            let value = param.next();
            params.insert(key.to_owned(), value.map(ToOwned::to_owned));
        }
        return Ok(UriQueryParams { params });
    }
}

pub fn get_query_params(request: &vercel_lambda::Request) -> Result<UriQueryParams, VercelError> {
    request
        .uri()
        .query()
        .ok_or_else(|| VercelError::new("Failed to parse query."))?
        .parse()
}
