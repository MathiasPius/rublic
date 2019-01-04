mod models;
mod auth;
mod domains;
mod users;
mod groups;

use actix_web::{Scope, FutureResponse, HttpResponse, AsyncResponder};
use futures::future::Future;
use crate::errors::ServiceError;
use crate::authorization::authorize;
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
        // Authorize with an empty vec will just ensure that *some* claims exist on the user
        // Whether they are adequate is decided on the endpoint
        .nested("/auth", auth::register)
        .nested("", |authorized| {
            authorized.middleware(authorize(&[]))
            .nested("/domains", domains::register)
            .nested("/users", users::register)
            .nested("/groups", groups::register)
        })
}