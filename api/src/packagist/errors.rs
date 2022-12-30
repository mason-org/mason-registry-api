use http::StatusCode;
use thiserror::Error;
use vercel_lambda::error::VercelError;

use crate::errors::ApiError;

#[derive(Error, Debug)]
pub enum PackagistError {
    #[error("The requested resource was not found when interfacing with Packagist API.")]
    ResourceNotFound { source: Option<reqwest::Error> },
    #[error("Client error. {:?}", source.status())]
    ClientError { source: reqwest::Error },
    #[error("Packagist API had a server error. {:?}", source.status())]
    ServerError { source: reqwest::Error },
    #[error("Network error. {:?}", source.status())]
    NetworkError { source: reqwest::Error },
}

impl ApiError for PackagistError {
    fn status_code(&self) -> http::StatusCode {
        match self {
            PackagistError::ResourceNotFound { .. } => StatusCode::NOT_FOUND,
            PackagistError::ClientError { .. } | PackagistError::NetworkError { .. } => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            PackagistError::ServerError { .. } => StatusCode::BAD_GATEWAY,
        }
    }
}

impl From<PackagistError> for VercelError {
    fn from(npm_error: PackagistError) -> Self {
        Self::new(&npm_error.to_string())
    }
}

impl From<reqwest::Error> for PackagistError {
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