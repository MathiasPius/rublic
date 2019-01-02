mod models;
mod domains;
mod users;
mod groups;

use actix_web::{Scope, FutureResponse, HttpResponse, AsyncResponder};
use futures::future::Future;
use crate::errors::ServiceError;
use super::app::AppState;

pub fn into_api_response<T: serde::Serialize>(response: impl Future<Item = T, Error = ServiceError> + 'static) 
    -> FutureResponse<HttpResponse> {
    response
        .and_then(|result|
            Ok(HttpResponse::Ok().json(result))
        )
        .from_err()
        .responder()
}

pub fn register(scope: Scope<AppState>) -> Scope<AppState> {
    scope
        .nested("/domains", |feature| domains::register(feature))
        .nested("/users", |feature| users::register(feature))
        .nested("/groups", |feature| groups::register(feature))
}