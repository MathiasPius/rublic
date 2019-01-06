use actix_web::{State, http::Method, Scope, HttpResponse, FutureResponse, Query, AsyncResponder};
use futures::future::Future;
use crate::errors::ServiceError;
use crate::app::AppState;
use crate::authorization::messages::*;
use crate::config::{JWT_ACCESS_LIFETIME, JWT_REFRESH_LIFETIME};
use super::models::*;
use super::{make_result, ResultType};

pub fn register(router: Scope<AppState>) -> Scope<AppState> {
    router
        .resource("/token", |r| {
            r.method(Method::POST).with_async(api_password_grant_query);
        })
}

fn api_password_grant_query((state, grant): (State<AppState>, Query<Grant>))
    -> FutureResponse<HttpResponse> {

    match grant.into_inner() {
        Grant::Password(pw) => {
            password_grant(state, pw.username, pw.password)
                .then(make_result(ResultType::Created)).responder()
        },
        Grant::Refresh(rf) => {
            state.authman.clone()
                .send(RefreshToken { 
                    token: rf.refresh_token.clone(),
                    lifetime: *JWT_ACCESS_LIFETIME
                }).flatten()
                .and_then(move |token| Ok(TokenResponse {
                    token_type: "bearer".into(),
                    expires_in: JWT_ACCESS_LIFETIME.num_seconds(),
                    access_token: token,
                    refresh_token: rf.refresh_token
                }))
                .then(make_result(ResultType::Created)).responder()
        }
    }   
}

fn password_grant(state: State<AppState>, username: String, password: String)
    -> impl Future<Item = TokenResponse, Error = ServiceError> {

    state.authman.clone()
        .send(AuthorizeUser{ friendly_name: username, password }).flatten()
        .and_then(move |claims| {
            state.authman.clone().send(BuildTokenFromClaims {
                lifetime: *JWT_ACCESS_LIFETIME,
                claims: claims.clone()
            }).flatten()
            .join(state.authman.clone().send(BuildTokenFromClaims {
                lifetime: *JWT_REFRESH_LIFETIME,
                claims
            }).flatten())
            .and_then(move |(access, refresh)| Ok(TokenResponse {
                token_type: "bearer".into(),
                expires_in: JWT_ACCESS_LIFETIME.num_seconds(),
                access_token: access,
                refresh_token: refresh
            }))
        })
}