use actix_web::{State, http::Method, Scope, HttpResponse, FutureResponse, Path};
use futures::future::Future;
use crate::app::AppState;
use crate::errors::ServiceError;
use super::into_api_response;
use super::models::*;
use crate::database::messages::*;

pub fn register(router: Scope<AppState>) -> Scope<AppState> {
    router
        .nested("/{fqdn}", |entry| {
            entry.resource("", |r| {
                r.method(Method::GET).with(api_get_domain);
                r.method(Method::POST).with(api_create_domain);
            })
        })
}

fn api_create_domain((fqdn, state): (Path<String>, State<AppState>))
    -> FutureResponse<HttpResponse> {

    into_api_response(state.db
        .send(CreateDomain { fqdn: fqdn.into_inner() }).flatten()
        .and_then(|domain| Ok(PluggableDomain {
            fqdn: domain.fqdn,
            id: domain.id,
            groups: None
        }))
    )
}

fn api_get_domain((fqdn, state): (Path<String>, State<AppState>))
    -> FutureResponse<HttpResponse> {
  
    into_api_response(get_domain_by_fqdn(state, fqdn.to_string()))
}

fn get_domains_groups(state: State<AppState>, id: String) 
    -> impl Future<Item = Vec<PluggableGroup>, Error = ServiceError> {
    state.db
        .send(GetGroupsByDomain { id }).flatten()
        .and_then(|groups| 
            Ok(groups.into_iter().map(|group| PluggableGroup {
                id: group.id,
                friendly_name: group.friendly_name,
                domains: None,
                users: None
            }).collect())
        )
}

fn get_domain_by_fqdn(state: State<AppState>, fqdn: String)
    -> impl Future<Item = PluggableDomain, Error = ServiceError> {

    state.db
        .send(GetDomainByFqdn { fqdn }).flatten()
        .and_then(|domain| 
            get_domains_groups(state, domain.id.clone())
                .and_then(|groups| Ok(PluggableDomain {
                    id: domain.id.clone(),
                    fqdn: domain.fqdn,
                    groups: Some(groups)
                }))
        )
}