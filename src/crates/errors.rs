use http::StatusCode;
use thiserror::Error;

use crate::errors::ApiError;

#[derive(Error, Debug)]
pub enum CratesError {
    #[error("The requested resource was not found when interfacing with Crates registry API.")]
    ResourceNotFound { source: Option<reqwest::Error> },
    #[error("Client error.")]
    ClientError { source: Option<reqwest::Error> },
    #[error("Crate registry API had a server error.")]
    ServerError { source: Option<reqwest::Error> },
    #[error("Network error.")]
    NetworkError { source: Option<reqwest::Error> },
}

impl ApiError for CratesError {
    fn status_code(&self) -> http::StatusCode {
        match self {
            CratesError::ResourceNotFound { .. } => StatusCode::NOT_FOUND,
            CratesError::ClientError { .. } | CratesError::NetworkError { .. } => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            CratesError::ServerError { .. } => StatusCode::BAD_GATEWAY,
        }
    }
}

impl From<reqwest::Error> for CratesError {
    fn from(req_error: reqwest::Error) -> Self {
        match req_error.status() {
            Some(reqwest::StatusCode::NOT_FOUND) => Self::ResourceNotFound {
                source: Some(req_error),
            },
            Some(status_code) if status_code.is_server_error() => Self::ServerError {
                source: Some(req_error),
            },
            Some(status_code) if status_code.is_client_error() => Self::ClientError {
                source: Some(req_error),
            },
            Some(_) | None => Self::NetworkError {
                source: Some(req_error),
            },
        }
    }
}

impl From<crates_io_api::Error> for CratesError {
    fn from(err: crates_io_api::Error) -> Self {
        match err {
            crates_io_api::Error::NotFound(_) => CratesError::ResourceNotFound { source: None },
            crates_io_api::Error::Http(err) => CratesError::NetworkError { source: Some(err) },
            crates_io_api::Error::Url(_) => CratesError::NetworkError { source: None },
            crates_io_api::Error::PermissionDenied(_) => CratesError::ClientError { source: None },
            crates_io_api::Error::JsonDecode(_) => CratesError::ServerError { source: None },
            crates_io_api::Error::Api(_) => CratesError::ServerError { source: None },
            _ => CratesError::ServerError { source: None },
        }
    }
}
