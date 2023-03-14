use http::StatusCode;
use thiserror::Error;

use crate::errors::ApiError;

#[derive(Error, Debug)]
pub enum RubyGemsError {
    #[error("The requested resource was not found when interfacing with RubyGems API.")]
    ResourceNotFound { source: Option<reqwest::Error> },
    #[error("Client error. {:?}", source.status())]
    ClientError { source: reqwest::Error },
    #[error("RubyGems API had a server error. {:?}", source.status())]
    ServerError { source: reqwest::Error },
    #[error("Network error. {:?}", source.status())]
    NetworkError { source: reqwest::Error },
}

impl ApiError for RubyGemsError {
    fn status_code(&self) -> http::StatusCode {
        match self {
            RubyGemsError::ResourceNotFound { .. } => StatusCode::NOT_FOUND,
            RubyGemsError::ClientError { .. } | RubyGemsError::NetworkError { .. } => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            RubyGemsError::ServerError { .. } => StatusCode::BAD_GATEWAY,
        }
    }
}

impl From<reqwest::Error> for RubyGemsError {
    fn from(req_error: reqwest::Error) -> Self {
        match req_error.status() {
            Some(reqwest::StatusCode::NOT_FOUND) => Self::ResourceNotFound {
                source: Some(req_error),
            },
            Some(status_code) if status_code.is_server_error() => {
                Self::ServerError { source: req_error }
            }
            Some(status_code) if status_code.is_client_error() => {
                Self::ClientError { source: req_error }
            }
            Some(_) | None => Self::NetworkError { source: req_error },
        }
    }
}
