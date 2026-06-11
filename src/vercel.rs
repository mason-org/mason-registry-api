use http::{
    HeaderValue, Response, StatusCode,
    header::{CACHE_CONTROL, CONTENT_TYPE, LOCATION},
};
use serde::Serialize;
use vercel_runtime::{Error as VercelError, ResponseBody};

use crate::{CacheControl, ErrResponse, errors::ApiError};

pub fn err_json<T: ApiError>(error: T) -> Result<Response<ResponseBody>, VercelError> {
    tracing::error!(%error, "API error");
    json_response(
        error.status_code(),
        CacheControl::NoStore,
        &ErrResponse {
            message: error.to_string(),
        },
    )
}

pub fn ok_json<T: Serialize>(
    data: T,
    cache: CacheControl,
) -> Result<Response<ResponseBody>, VercelError> {
    json_response(StatusCode::OK, cache, &data)
}

pub fn redirect<S: AsRef<str>>(
    to: S,
    cache: CacheControl,
) -> Result<Response<ResponseBody>, VercelError> {
    Ok(Response::builder()
        .status(StatusCode::TEMPORARY_REDIRECT)
        .header(CACHE_CONTROL, cache.get_header())
        .header(LOCATION, HeaderValue::from_str(to.as_ref())?)
        .body(().into())?)
}

pub fn json_response<T: Serialize>(
    status: StatusCode,
    cache: CacheControl,
    data: &T,
) -> Result<Response<ResponseBody>, VercelError> {
    Ok(Response::builder()
        .status(status)
        .header(CONTENT_TYPE, "application/json")
        .header(CACHE_CONTROL, cache.get_header())
        .body(serde_json::to_string_pretty(data)?.into())?)
}

pub fn method_not_allowed() -> Result<Response<ResponseBody>, VercelError> {
    Ok(Response::builder()
        .status(StatusCode::METHOD_NOT_ALLOWED)
        .body(().into())?)
}
