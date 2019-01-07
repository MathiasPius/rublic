// errors.rs
use diesel::result::{Error, DatabaseErrorKind};
use std::convert::From;

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


impl From<actix::MailboxError> for ServiceError {
    fn from(_: actix::MailboxError) -> Self {
        ServiceError::InternalServerError
    }
}

impl From<diesel::result::Error> for ServiceError {
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

impl From<crate::certman::errors::Error> for ServiceError {
    fn from(_: crate::certman::errors::Error) -> Self {
        ServiceError::InternalServerError
    }
}

impl From<crate::database::errors::Error> for ServiceError {
    fn from(_: crate::database::errors::Error) -> Self {
        ServiceError::InternalServerError
    }
}

impl From<crate::authorization::errors::Error> for ServiceError {
    fn from(_: crate::authorization::errors::Error) -> Self {
        ServiceError::InternalServerError
    }
}

impl From<std::io::Error> for ServiceError {
    fn from(_e: std::io::Error) -> Self {
        ServiceError::InternalServerError
    }
}

impl From<jwt::errors::Error> for ServiceError {
    fn from(_e: jwt::errors::Error) -> Self {
        ServiceError::InternalServerError
    }
}