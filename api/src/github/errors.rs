use http::StatusCode;
use thiserror::Error;
use vercel_lambda::error::VercelError;

use crate::errors::ApiError;

#[derive(Error, Debug)]
pub enum GitHubError {
    #[error("The requested resource was not found when interfacing with GitHub's API.")]
    ResourceNotFound { source: Option<reqwest::Error> },
    #[error("Client error. {:?}", source.status())]
    ClientError { source: reqwest::Error },
    #[error("GitHub's API had a server error. {:?}", source.status())]
    ServerError { source: reqwest::Error },
    #[error("Network error. {:?}", source.status())]
    NetworkError { source: reqwest::Error },
}

impl ApiError for GitHubError {
    fn status_code(&self) -> StatusCode {
        match self {
            GitHubError::ResourceNotFound { .. } => StatusCode::NOT_FOUND,
            GitHubError::ClientError { .. } | GitHubError::NetworkError { .. } => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            GitHubError::ServerError { .. } => StatusCode::BAD_GATEWAY,
        }
    }
}

impl From<GitHubError> for VercelError {
    fn from(github_error: GitHubError) -> Self {
        Self::new(&github_error.to_string())
    }
}

impl From<reqwest::Error> for GitHubError {
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
