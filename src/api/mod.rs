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

pub fn api_result<T: serde::Serialize>(response: impl Future<Item = T, Error = ServiceError> + 'static) 
    -> FutureResponse<HttpResponse> {
    Box::new(response
        .and_then(|result|
            Ok(ApiResult::<T>::Data(result).into())
        ).from_err())
}

pub enum ResultType {
    Data,
    Created
}

pub fn make_result<T: serde::Serialize>(result_type: ResultType) 
    -> Box<impl FnOnce(Result<T, ServiceError>) -> Result<HttpResponse, actix_web::Error>> {
    return Box::new(move |result: Result<T, ServiceError>| {
        match result {
            Ok(data) => {
                match result_type {
                    ResultType::Data => Ok(ApiResult::<T>::Data(data).into()),
                    ResultType::Created => Ok(ApiResult::<T>::Created(data).into())
                }
            },
            Err(e) => Err(e.into())
        }
    })
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

impl<T> Into<HttpResponse> for ApiResult<T> 
    where T: serde::Serialize 
{
    fn into(self) -> HttpResponse {
        match self {
            ApiResult::<T>::Data(data) => {
                HttpResponse::Ok().json(data)
            },
            ApiResult::<T>::Created(data) => {
                HttpResponse::Created().json(data)
            },
            ApiResult::<T>::Acknowledged => {
                HttpResponse::Ok().finish()
            },
            ApiResult::<T>::Error(e) => {
                e.error_response()
            }
        }
    }
}