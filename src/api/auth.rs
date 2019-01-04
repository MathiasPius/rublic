use chrono::Duration;
use actix_web::{State, http::Method, Scope, HttpResponse, FutureResponse, Query, AsyncResponder};
use actix_web_httpauth::extractors::basic::BasicAuth;
use futures::future::Future;
use crate::errors::ServiceError;
use crate::app::AppState;
use crate::authorization::messages::*;
use super::models::*;

pub fn register(router: Scope<AppState>) -> Scope<AppState> {
    router
        .resource("/token", |r| {
            r.method(Method::POST).with_async(api_password_grant_query);
        })
}

fn api_password_grant_query((state, grant): (State<AppState>, Query<PasswordGrant>))
    -> FutureResponse<HttpResponse> {

    let grant = grant.into_inner();

    password_grant(state, grant.username, grant.password)
        .and_then(|response| {
            Ok(HttpResponse::Ok().json(response))
        })
        .from_err()
        .responder()
}

fn password_grant(state: State<AppState>, username: String, password: String)
    -> impl Future<Item = TokenResponse, Error = ServiceError> {
    let access_token_lifetime = Duration::hours(1);
    let refresh_token_lifetime = Duration::days(30);
    println!("authorizing {}, {}", &username, &password);

    state.authman.clone()
        .send(AuthorizeUser{ friendly_name: username, password: password }).flatten()
        .and_then(move |claims| {
            state.authman.clone().send(BuildTokenFromClaims {
                lifetime: access_token_lifetime,
                claims: claims.clone()
            }).flatten()
            .join(state.authman.clone().send(BuildTokenFromClaims {
                lifetime: refresh_token_lifetime,
                claims
            }).flatten())
            .and_then(move |(access, refresh)| Ok(TokenResponse {
                token_type: "bearer".into(),
                expires_in: access_token_lifetime.num_seconds(),
                access_token: access,
                refresh_token: refresh
            }))
        })
}