use std::fmt::Display;

use http::StatusCode;
use url::ParseError;

#[derive(thiserror::Error, Debug)]
pub enum CoreError {
    UrlParse(url::ParseError),
}

impl Display for CoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CoreError::UrlParse(err) => f.write_fmt(format_args!("{}", err)),
        }
    }
}

impl From<ParseError> for CoreError {
    fn from(value: ParseError) -> Self {
        Self::UrlParse(value)
    }
}

pub trait ApiError: Display {
    fn status_code(&self) -> StatusCode;
}

impl ApiError for ParseError {
    fn status_code(&self) -> StatusCode {
        todo!()
    }
}

impl ApiError for CoreError {
    fn status_code(&self) -> StatusCode {
        match self {
            CoreError::UrlParse(_) => StatusCode::BAD_REQUEST,
        }
    }
}
