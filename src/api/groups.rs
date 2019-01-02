use actix_web::{State, http::Method, Scope, HttpResponse, FutureResponse, Path, Json};
use futures::future::Future;
use crate::app::AppState;
use crate::errors::ServiceError;
use super::into_api_response;
use super::models::*;
use crate::database::messages::*;

pub fn register(router: Scope<AppState>) -> Scope<AppState> {
    router
        .nested("/{group_id}", |entry| {
            entry.resource("", |r| {
                r.method(Method::GET).with(api_get_group);
            })
        })
        .resource("", |r| {
            r.method(Method::POST).with(api_create_group);
        })
}

fn api_create_group((group, state): (Json<NewGroupRequest>, State<AppState>))
    -> FutureResponse<HttpResponse> {

    into_api_response(state.db
        .send(CreateGroup { friendly_name: group.friendly_name.clone() }).flatten()
        .and_then(|group| Ok(PluggableGroup {
            id: group.id,
            friendly_name: group.friendly_name,
            domains: None
        }))
    )
}

fn api_get_group((group_id, state): (Path<String>, State<AppState>))
    -> FutureResponse<HttpResponse> {
  
    into_api_response(get_group(state, group_id.to_string()))
}

fn get_group(state: State<AppState>, id: String)
    -> impl Future<Item = PluggableGroup, Error = ServiceError> {

    state.db
        .send(GetGroup { id }).flatten()
        .and_then(|group| Ok(PluggableGroup {
            id: group.id,
            friendly_name: group.friendly_name,
            domains: None
        }))
}