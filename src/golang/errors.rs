use http::StatusCode;
use thiserror::Error;
use vercel_lambda::error::VercelError;

use crate::errors::ApiError;

#[derive(Error, Debug)]
pub enum GolangError {
    #[error("The requested resource was not found when interfacing with Golang Proxy API.")]
    ResourceNotFound { source: Option<reqwest::Error> },
    #[error("Client error. {:?}", source.status())]
    ClientError { source: reqwest::Error },
    #[error("Golang Proxy API had a server error. {:?}", source.status())]
    ServerError { source: reqwest::Error },
    #[error("Network error. {:?}", source.status())]
    NetworkError { source: reqwest::Error },
}

impl ApiError for GolangError {
    fn status_code(&self) -> http::StatusCode {
        match self {
            GolangError::ResourceNotFound { .. } => StatusCode::NOT_FOUND,
            GolangError::ClientError { .. } | GolangError::NetworkError { .. } => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            GolangError::ServerError { .. } => StatusCode::BAD_GATEWAY,
        }
    }
}

impl From<GolangError> for VercelError {
    fn from(error: GolangError) -> Self {
        Self::new(&error.to_string())
    }
}

impl From<reqwest::Error> for GolangError {
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
