use actix_web::{AsyncResponder, FutureResponse, HttpResponse, /*Json,*/ ResponseError, State, http::Method, Scope, Path};
use futures::future::*;

use crate::errors::ServiceError;
use crate::app::{AppState, RublicFeatureRouter};
use crate::domains::models::{internal::*, external::*};

pub struct RublicDomainsRouter { }
impl RublicFeatureRouter for RublicDomainsRouter {
    fn register(router: Scope<AppState>) -> Scope<AppState> {
        router
            .nested("/domains", |domains| {
                domains.nested("/{domain_entry_id}", |entry| {
                    entry.resource("", |r| {
                        r.method(Method::GET).with(get_domain_entry)
                    })
                })
                .resource("", |r| {
                    r.method(Method::GET).with(get_all_domain_entries)
                })
            })
            
            .nested("/domaingroups", |domaingroups| {
                domaingroups
                    .resource("/{domain_group_id}", |r| {
                        r.method(Method::GET).with(get_expanded_domain_group)
                    })
                    .resource("", |r| {
                        r.method(Method::GET).with(get_all_domain_groups)
                    })
            })
    }
}


fn get_domains_groups(state: State<AppState>, id: String) -> impl Future<Item = Vec<SimpleDomainGroup>, Error = ServiceError> {
    state.db
        .send(GetDomainGroupsByDomainEntry { id: id })
        .flatten()
        .and_then(|groups| 
            Ok(groups.into_iter().map(|group| SimpleDomainGroup {
                id: group.id,
                friendly_name: group.friendly_name
            }).collect())
        )
}

fn get_domain_entry((domain_entry_fqdn, state): (Path<String>, State<AppState>))
    -> FutureResponse<HttpResponse> {
  
    state.db.send(GetDomainEntryByFqdn{ fqdn: domain_entry_fqdn.into_inner() })
        .flatten()
        .and_then(|domain| 
            get_domains_groups(state, domain.id.clone())
                .and_then(|groups| Ok(PluggableDomainEntry {
                    id: domain.id.clone(),
                    fqdn: domain.fqdn,
                    groups: Some(groups)
                }))
        )
        .and_then(|result|
            Ok(HttpResponse::Ok().json(result))
        )
        .from_err()
        .responder()
}

fn get_all_domain_entries(state: State<AppState>) -> FutureResponse<HttpResponse>
{
    state
        .db
        .send(GetAllDomainEntries {})
        .from_err()
        .and_then(|db_response| match db_response {
            Ok(domains) => Ok(HttpResponse::Ok().json(domains)),
            Err(err) => Ok(err.error_response())
        }).responder()
}

fn get_expanded_domain_group((domain_group_id, state): (Path<String>, State<AppState>))
    -> FutureResponse<HttpResponse> {

    state
        .db
        .send(GetExpandedDomainGroup { id: domain_group_id.into_inner() })
        .from_err()
        .and_then(|db_response| match db_response {
            Ok(group) => Ok(HttpResponse::Ok().json(group)),
            Err(err) => Ok(err.error_response())
        }).responder()
}

fn get_all_domain_groups(state: State<AppState>) -> FutureResponse<HttpResponse>
{
    state
        .db
        .send(GetAllDomainGroups {})
        .from_err()
        .and_then(|db_response| match db_response {
            Ok(groups) => Ok(HttpResponse::Ok().json(groups)),
            Err(err) => Ok(err.error_response())
        }).responder()
}