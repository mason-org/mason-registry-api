use http::header::{CACHE_CONTROL, CONTENT_TYPE};
use serde::Serialize;
use vercel_lambda::{error::VercelError, Body, Response};

pub fn json<T: Serialize>(data: T) -> Result<Response<Body>, VercelError> {
    Response::builder()
        .status(200)
        .header(CONTENT_TYPE, "application/json")
        .header(CACHE_CONTROL, "public, s-maxage=1800")
        .body(Body::Text(
            serde_json::to_string_pretty(&data)
                .map_err(|_| VercelError::new("Failed to serialize."))?,
        ))
        .map_err(|_| VercelError::new("Failed to build response."))
}
