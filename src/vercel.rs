use http::{
    header::{CACHE_CONTROL, CONTENT_TYPE},
    Response, StatusCode,
};
use serde::Serialize;
use vercel_runtime::{Body, Error as VercelError, Request};

use crate::{errors::ApiError, CacheControl, ErrResponse};

pub fn err_json<T: ApiError>(error: T) -> Result<Response<Body>, VercelError> {
    tracing::error!(%error, "API error");
    json_response(
        error.status_code(),
        CacheControl::NoStore,
        &ErrResponse {
            message: error.to_string(),
        },
    )
}

pub fn ok_json<T: Serialize>(data: T, cache: CacheControl) -> Result<Response<Body>, VercelError> {
    json_response(StatusCode::OK, cache, &data)
}

pub fn json_response<T: Serialize>(
    status: StatusCode,
    cache: CacheControl,
    data: &T,
) -> Result<Response<Body>, VercelError> {
    Ok(Response::builder()
        .status(status)
        .header(CONTENT_TYPE, "application/json")
        .header(
            CACHE_CONTROL,
            match cache {
                CacheControl::NoStore => "no-store",
                CacheControl::PublicShort => "s-maxage=60, stale-while-revalidate=120",
                CacheControl::PublicMedium => "s-maxage=1800",
            },
        )
        .body(Body::Text(serde_json::to_string_pretty(data)?))?)
}

pub fn parse_url(request: &Request) -> Result<url::Url, crate::errors::CoreError> {
    Ok(url::Url::parse(&request.uri().to_string())?)
}
