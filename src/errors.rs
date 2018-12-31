// errors.rs
use serde_derive::{Serialize, Deserialize};
use actix_web::{error::ResponseError, HttpResponse};


#[allow(dead_code)]
#[derive(Fail, Debug)]
pub enum ServiceError {
    #[fail(display = "Internal Server Error")]
    InternalServerError,

    #[fail(display = "BadRequest: {}", _0)]
    BadRequest(String),

    #[fail(display = "NotFound: {}", _0)]
    NotFound(String),
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
            ServiceError::BadRequest(ref message) => HttpResponse::BadRequest().json(ApiError { error: message.clone() }),
            ServiceError::NotFound(ref message) => HttpResponse::NotFound().json(ApiError { error: message.clone() }),
        }
    }
}