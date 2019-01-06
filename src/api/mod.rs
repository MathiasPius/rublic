mod models;
mod auth;
mod domains;
mod users;
mod groups;

use actix_web::{Scope, FutureResponse, ResponseError, HttpResponse, AsyncResponder, HttpRequest, http::StatusCode, Responder};
use futures::future::Future;
use serde_derive::{Serialize};
use crate::errors::ServiceError;
use super::app::AppState;

pub fn register(scope: Scope<AppState>) -> Scope<AppState> {
    scope
        // Authorize with an empty vec will just ensure that *some* claims exist on the user
        // Whether they are adequate is decided on the endpoint
        .nested("/auth", auth::register)
        .nested("/domains", domains::register)
        .nested("/users", users::register)
        .nested("/groups", groups::register)
}

pub fn into_api_response<T: serde::Serialize>(response: impl Future<Item = T, Error = ServiceError> + 'static) 
    -> FutureResponse<HttpResponse> {
    response
        .and_then(|result|
            Ok(HttpResponse::Ok().json(result))
        )
        .from_err()
        .responder()
}


pub enum ApiResult<T> 
    where T: serde::Serialize 
{
    Created(T),
    Data(T),
    Acknowledged,
    Error(ServiceError)
}

impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::InternalServerError().finish()
    }
}

impl<T> Responder for ApiResult<T> 
    where T: serde::Serialize 
{
    type Item = HttpResponse;
    type Error = actix_web::error::Error;

    fn respond_to<S>(self, req: &HttpRequest<S>) -> Result<HttpResponse, Self::Error> {
        match self {
            ApiResult::<T>::Data(data) => {
                Ok(req.build_response(StatusCode::OK)
                    .content_type("application/json")
                    .json(data))
            },
            ApiResult::<T>::Created(data) => {
                Ok(req.build_response(StatusCode::CREATED)
                    .content_type("application/json")
                    .json(data))
            },
            ApiResult::<T>::Acknowledged => {
                Ok(req.build_response(StatusCode::OK).finish())
            },
            ApiResult::<T>::Error(e) => {
                Ok(e.error_response())
            }
        }
    }
}