use std::fmt::Display;

use http::StatusCode;

pub trait ApiError: Display {
    fn status_code(&self) -> StatusCode;
}
