use actix_web::{State, http::Method, Scope, HttpResponse, FutureResponse, Path, Json};
use futures::future::Future;
use crate::app::AppState;
use crate::errors::ServiceError;
use crate::database::messages::*;
use crate::cryptoutil::CryptoUtil;
use super::into_api_response;
use super::models::*;


pub fn register(router: Scope<AppState>) -> Scope<AppState> {
    router
        .nested("/{user_id}", |entry| {
            entry.resource("", |r| {
                r.method(Method::GET).with(api_get_user);
            })
        })
        .resource("", |r| {
            r.method(Method::POST).with(api_create_user);
        })
}

fn api_create_user((new_user, state): (Json<NewUserRequest>, State<AppState>)) 
    -> FutureResponse<HttpResponse> {
    
    let key = CryptoUtil::generate_key();
    let hashed_key = CryptoUtil::hash_key(&key);

    into_api_response(state.db
        .send(CreateUser { friendly_name: new_user.into_inner().friendly_name, hashed_key }).flatten()
        .and_then(|user| Ok(PluggableUser {
            id: user.id,
            friendly_name: user.friendly_name,
            secret_key: Some(key)
        }))
    )
}

fn api_get_user((user_id, state): (Path<String>, State<AppState>))
    -> FutureResponse<HttpResponse> {
  
    into_api_response(get_user(state, user_id.to_string()))
}

fn get_user(state: State<AppState>, id: String)
    -> impl Future<Item = PluggableUser, Error = ServiceError> {

    state.db
        .send(GetUserById { id }).flatten()
        .and_then(|user| Ok(PluggableUser {
            id: user.id,
            friendly_name: user.friendly_name,
            secret_key: None
        }))
}