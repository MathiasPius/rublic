use actix_web::{State, http::Method, Scope, HttpResponse, FutureResponse, Path};
use futures::future::Future;
use crate::app::AppState;
use crate::errors::ServiceError;
use super::into_api_response;
use super::models::*;
use crate::database::messages::*;

pub fn register(router: Scope<AppState>) -> Scope<AppState> {
    router
        .nested("/{domain_entry_id}", |entry| {
            entry.resource("", |r| {
                r.method(Method::GET).with(get_domain_entry)
            })
        })
}

fn get_domains_groups(state: State<AppState>, id: String) 
    -> impl Future<Item = Vec<PluggableGroup>, Error = ServiceError> {
    state.db
        .send(GetGroupsByDomain { id }).flatten()
        .and_then(|groups| 
            Ok(groups.into_iter().map(|group| PluggableGroup {
                id: group.id,
                friendly_name: group.friendly_name,
                domains: None
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

fn get_domain_entry((domain_entry_fqdn, state): (Path<String>, State<AppState>))
    -> FutureResponse<HttpResponse> {
  
    into_api_response(get_domain_by_fqdn(state, domain_entry_fqdn.to_string()))
}