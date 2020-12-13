use core::fmt;
use std::collections::HashMap;
use std::fmt::Display;

use actix_web::{error::ResponseError, error, HttpRequest, HttpResponse};
use actix_web::error::JsonPayloadError;
use serde::export::Formatter;

use crate::{common};

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub enum ApiError {
    BadRequest(String),
    // BlockingError(String),
    // CacheError(String),
    // CannotDecodeJwtToken(String),
    // CannotEncodeJwtToken(String),
    InternalServerError(String),
    NotFound(String),

    DbError(String),
    // ParseError(String),
    // PoolError(String),
    // #[display(fmt = "")]
    // ValidationError(Vec<String>),
    Unauthorized(String),
}

impl Display for ApiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "my display")
    }
}

impl From<sqlx::Error> for ApiError {
    #[inline]
    fn from(err: sqlx::Error) -> Self {
        ApiError::DbError(err.to_string())
    }
}

// impl From<actix_web::Error> for ApiError {
//     #[inline]
//     fn from(err: actix_web::Error) -> Self {
//         ApiError::InternalServerError(err.to_string())
//     }
// }

/// Automatically convert ApiErrors to external Response Errors
impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ApiError::BadRequest(error) => HttpResponse::BadRequest().json(make_error_object(error)),
            ApiError::NotFound(message) => HttpResponse::NotFound().json(make_error_object(message)),
            ApiError::InternalServerError(error) => make_internal_error_json_response(error),
            ApiError::DbError(error) => HttpResponse::InternalServerError().json(make_error_object(error)),
            ApiError::Unauthorized(error) => HttpResponse::Unauthorized().json(make_error_object(error)),
            // ApiError::ValidationError(errors) => {
            //     HttpResponse::UnprocessableEntity().json::<ErrorResponse>(errors.to_vec().into())
            // }
            // ApiError::Unauthorized(error) => {
            //     HttpResponse::Unauthorized().json::<ErrorResponse>(error.into())
            // }
            // _ => HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}

pub fn make_error_object(reason: &str) -> HashMap<String, String> {
    let params = vec![(String::from("reason"), reason.to_string())];
    return common::make_api_response(false, params);
}

pub fn make_internal_error_json_response(msg: &str) -> HttpResponse {
    HttpResponse::InternalServerError().json(make_error_object(msg))
}

pub fn json_error_handler(err: error::JsonPayloadError, _req: &HttpRequest) -> error::Error {
    let reason = err.to_string();
    let api_err_obj = make_error_object(reason.as_str());
    let resp = match &err {
        JsonPayloadError::ContentType => {
            HttpResponse::UnsupportedMediaType().json(api_err_obj)
        }
        JsonPayloadError::Deserialize(json_err) if json_err.is_data() => {
            HttpResponse::UnprocessableEntity().json(api_err_obj)
        }
        _ => HttpResponse::BadRequest().json(api_err_obj),
    };
    error::InternalError::from_response(err, resp).into()
}
