use actix::Addr;
use actix_web::{State, http::Method, Scope, HttpResponse, FutureResponse, Path, Json, AsyncResponder};
use futures::future::Future;
use crate::app::AppState;
use crate::errors::ServiceError;
use crate::database::DbExecutor;
use crate::database::messages::*;
use crate::authorization::ResourceAuthorization;
use crate::cryptoutil::CryptoUtil;
use super::{make_result, ResultType};
use super::models::*;


pub fn register(router: Scope<AppState>) -> Scope<AppState> {
    router
        .authorize_resource("*", "*")
        .nested("/{user_id}", |entry| {
            entry.resource("", |r| {
                r.method(Method::GET).with_async(api_get_user);
            })
        })
        .resource("", |r| {
            r.method(Method::POST).with_async(api_create_user);
        })
}

fn api_create_user((new_user, state): (Json<NewUserRequest>, State<AppState>)) 
    -> FutureResponse<HttpResponse> {
    
    let key = CryptoUtil::generate_key();
    let hashed_key = CryptoUtil::hash_key(&key);

    state.db
        .send(CreateUser { 
            friendly_name: new_user.friendly_name.clone(), 
            hashed_key 
        }).flatten().from_err()
        .and_then(|user| Ok(PluggableUser {
            id: user.id,
            friendly_name: user.friendly_name,
            secret_key: Some(key),
            groups: None
        }))
        .then(make_result(ResultType::Created)).responder()
}

fn api_get_user((user_id, state): (Path<String>, State<AppState>))
    -> FutureResponse<HttpResponse> {
  
    get_user(state.db.clone(), user_id.to_string())
        .then(make_result(ResultType::Created)).responder()
}

fn get_user(db: Addr<DbExecutor>, id: String)
    -> impl Future<Item = PluggableUser, Error = ServiceError> {

    db.clone()
        .send(GetUser { id }).flatten().from_err()
        .and_then(move |user| {
            get_user_groups(db.clone(), user.id.clone())
                .and_then(|groups| {
                    Ok(PluggableUser {
                        id: user.id,
                        friendly_name: user.friendly_name,
                        secret_key: None,
                        groups: Some(groups)
                    })
                })
        })
}

fn get_user_groups(db: Addr<DbExecutor>, id: String) 
    -> impl Future<Item = Vec<PluggableGroup>, Error = ServiceError> {
    db
        .send(GetGroupsByUser { id }).flatten()
        .map_err(|e| e.into())
        .and_then(|groups| 
            Ok(groups.into_iter().map(|group| PluggableGroup {
                id: group.id,
                friendly_name: group.friendly_name,
                users: None,
                domains: None
            }).collect())
        )
}