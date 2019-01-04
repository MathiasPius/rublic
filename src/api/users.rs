use actix::Addr;
use actix_web::{State, http::Method, Scope, HttpResponse, FutureResponse, Path, Json};
use futures::future::Future;
use crate::app::AppState;
use crate::errors::ServiceError;
use crate::database::DbExecutor;
use crate::database::messages::*;
use crate::authorization::authorize;
use crate::cryptoutil::CryptoUtil;
use super::into_api_response;
use super::models::*;


pub fn register(router: Scope<AppState>) -> Scope<AppState> {
    router
        .middleware(authorize(&[("*", "*")]))
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

    into_api_response(state.db
        .send(CreateUser { friendly_name: new_user.friendly_name.clone(), hashed_key }).flatten()
        .and_then(|user| Ok(PluggableUser {
            id: user.id,
            friendly_name: user.friendly_name,
            secret_key: Some(key),
            groups: None
        }))
    )
}

fn api_get_user((user_id, state): (Path<String>, State<AppState>))
    -> FutureResponse<HttpResponse> {
  
    into_api_response(get_user(state.db.clone(), user_id.to_string()))
}

fn get_user(db: Addr<DbExecutor>, id: String)
    -> impl Future<Item = PluggableUser, Error = ServiceError> {

    db.clone()
        .send(GetUser { id }).flatten()
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
        .and_then(|groups| 
            Ok(groups.into_iter().map(|group| PluggableGroup {
                id: group.id,
                friendly_name: group.friendly_name,
                users: None,
                domains: None
            }).collect())
        )
}