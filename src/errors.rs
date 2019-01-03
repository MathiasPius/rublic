// errors.rs
use serde_derive::{Serialize, Deserialize};
use actix_web::{error::ResponseError, HttpResponse};
use diesel::result::{Error, DatabaseErrorKind};

#[allow(dead_code)]
#[derive(Fail, Debug)]
pub enum ServiceError {
    #[fail(display = "Internal Server Error")]
    InternalServerError,

    #[fail(display = "BadRequest: {}", _0)]
    BadRequest(String),

    #[fail(display = "NotFound: {}", _0)]
    NotFound(String),

    #[fail(display = "Conflict: {}", _0)]
    Conflict(String),

    #[fail(display = "Unauthorized")]
    Unauthorized
}

#[derive(Serialize, Deserialize)]
pub struct ApiError {
    pub error: String
}

// impl ResponseError trait allows to convert our errors into http responses with appropriate data
impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            ServiceError::InternalServerError => {
                HttpResponse::InternalServerError().json(ApiError { error: "Internal Server Error".into() })
            },
            ServiceError::Unauthorized => {
                HttpResponse::Unauthorized().json(ApiError { error: "Unauthorized".into() })
            },
            ServiceError::BadRequest(ref message) => HttpResponse::BadRequest().json(ApiError { error: message.clone() }),
            ServiceError::NotFound(ref message) => HttpResponse::NotFound().json(ApiError { error: message.clone() }),
            ServiceError::Conflict(ref message) => HttpResponse::Conflict().json(ApiError { error: message.clone() }),
        }
    }
}

impl std::convert::From<actix::MailboxError> for ServiceError {
    fn from(_: actix::MailboxError) -> Self {
        ServiceError::InternalServerError
    }
}

impl std::convert::From<diesel::result::Error> for ServiceError {
    fn from(e: diesel::result::Error) -> Self {
        match e {
            Error::DatabaseError(kind, info) => match kind {
                DatabaseErrorKind::UniqueViolation => {
                    ServiceError::Conflict(info.message().to_string())
                },
                _ => ServiceError::InternalServerError
            },
            _ => ServiceError::InternalServerError
        }
    }
}

impl std::convert::From<std::io::Error> for ServiceError {
    fn from(_e: std::io::Error) -> Self {
        ServiceError::InternalServerError
    }
}