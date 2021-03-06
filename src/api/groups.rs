use actix::Addr;
use actix_web::{State, http::Method, Scope, HttpResponse, FutureResponse, Path, Json, AsyncResponder};
use futures::future::{join_all, Future};
use crate::app::AppState;
use crate::errors::ServiceError;
use crate::database::messages::*;
use crate::database::DbExecutor;
use crate::authorization::ResourceAuthorization;
use super::{make_result, ResultType};
use super::models::*;

pub fn register(router: Scope<AppState>) -> Scope<AppState> {
    router
        .authorize_resource("*", "*")
        .nested("/{group_id}", |entry| {
            entry.resource("", |r| {
                r.method(Method::GET).with_async(api_get_group);
            })
            .nested("/users", |users| {
                users.resource("", |r| {
                    r.method(Method::PUT).with_async(api_set_group_users);
                    r.method(Method::GET).with_async(api_get_group_users);
                })
            })
            .nested("/domains", |domains| {
                domains.resource("", |r| {
                    r.method(Method::PUT).with_async(api_set_group_domains);
                    r.method(Method::GET).with_async(api_get_group_domains);
                })
            })
        })
        .resource("", |r| {
            r.method(Method::POST).with_async(api_create_group);
            r.method(Method::GET).with_async(api_get_groups);
        })
}

fn api_get_groups(state: State<AppState>) 
    -> FutureResponse<HttpResponse> {
    state.db.clone()
        .send(GetGroups {}).flatten().from_err()
        .and_then(move |groups|
            join_all(groups.into_iter().map(move |group|
                get_group(state.db.clone(), group.id)
            ))
        )
        .then(make_result(ResultType::Created)).responder()
}

fn api_get_group_users((group_id, state): (Path<String>, State<AppState>))
    -> FutureResponse<HttpResponse> {
    get_group_users(state.db.clone(), group_id.into_inner())
        .then(make_result(ResultType::Created)).responder()
}

fn api_get_group_domains((group_id, state): (Path<String>, State<AppState>))
    -> FutureResponse<HttpResponse> {
    get_group_domains(state.db.clone(), group_id.into_inner())
        .then(make_result(ResultType::Created)).responder()
}

fn api_set_group_users((group_id, users, state): (Path<String>, Json<Vec<String>>, State<AppState>))
    -> FutureResponse<HttpResponse> {
        
    state.db.clone()
        .send(SetGroupUsers { 
            user_ids: users.into_inner(), 
            group_id: group_id.clone() 
        }).flatten().from_err()
        .and_then(move |_| {
            get_group(state.db.clone(), group_id.into_inner())
        })
        .then(make_result(ResultType::Created)).responder()
}

fn api_set_group_domains((group_id, fqdns, state): (Path<String>, Json<Vec<String>>, State<AppState>))
    -> FutureResponse<HttpResponse> {

    state.db.clone()
        .send(SetGroupDomains { 
            fqdns: fqdns.into_inner(), 
            group_id: group_id.clone() 
        }).flatten().from_err()
        .and_then(move |_| {
            get_group(state.db.clone(), group_id.into_inner())
        })
        .then(make_result(ResultType::Created)).responder()
}

fn api_create_group((group, state): (Json<NewGroupRequest>, State<AppState>))
    -> FutureResponse<HttpResponse> {

    state.db.clone()
        .send(CreateGroup { friendly_name: group.friendly_name.clone() }).flatten().from_err()
        .and_then(|group| Ok(PluggableGroup {
            id: group.id,
            friendly_name: group.friendly_name,
            domains: Some(Vec::new()),
            users: Some(Vec::new())
        }))
        .then(make_result(ResultType::Created)).responder()
}

fn api_get_group((group_id, state): (Path<String>, State<AppState>))
    -> FutureResponse<HttpResponse> {
  
    get_group(state.db.clone(), group_id.to_string())
        .then(make_result(ResultType::Created)).responder()
}

fn get_group(db: Addr<DbExecutor>, id: String)
    -> impl Future<Item = PluggableGroup, Error = ServiceError> {

    db.clone()
        .send(GetGroup { id: id.clone() }).flatten().from_err()
        .join3(
            get_group_users(db.clone(), id.clone()),
            get_group_domains(db.clone(), id.clone())
        )
        .and_then(|(group, users, domains)| {
            Ok(PluggableGroup {
                id: group.id,
                friendly_name: group.friendly_name,
                domains: Some(domains),
                users: Some(users)
            })
        })
}

fn get_group_users(db: Addr<DbExecutor>, id: String) 
    -> impl Future<Item = Vec<PluggableUser>, Error = ServiceError> {
    db
        .send(GetUsersByGroup { id }).flatten()
        .map_err(|e| e.into())
        .and_then(|users| 
            Ok(users.into_iter().map(|user| PluggableUser {
                id: user.id,
                friendly_name: user.friendly_name,
                secret_key: None,
                groups: None
            }).collect())
        )
}

fn get_group_domains(db: Addr<DbExecutor>, id: String) 
    -> impl Future<Item = Vec<PluggableDomain>, Error = ServiceError> {
    db
        .send(GetDomainsByGroup { id }).flatten()
        .map_err(|e| e.into())
        .and_then(|domains| 
            Ok(domains.into_iter().map(|domain| PluggableDomain {
                id: domain.id,
                fqdn: domain.fqdn,
                groups: None,
                latest_certs: None
            }).collect())
        )
}