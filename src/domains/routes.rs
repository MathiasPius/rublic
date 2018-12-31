use actix_web::{AsyncResponder, FutureResponse, HttpResponse, /*Json,*/ ResponseError, State, http::Method, Scope, Path};
use futures::future::Future;

use crate::app::{AppState, RublicFeatureRouter};
use crate::domains::models::{internal::*, /*external::**/};

pub struct RublicDomainsRouter { }
impl RublicFeatureRouter for RublicDomainsRouter {
    fn register(router: Scope<AppState>) -> Scope<AppState> {
        router
            .nested("/domains", |domains| {
                domains.resource("/{domain_entry_id}", |r| {
                    r.method(Method::GET).with(get_expanded_domain_entry)
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

fn get_expanded_domain_entry((domain_entry_id, state): (Path<String>, State<AppState>)) 
    -> FutureResponse<HttpResponse> {

    state
        .db
        .send(GetExpandedDomainEntry { id: domain_entry_id.into_inner() })
        .from_err()
        .and_then(|db_response| match db_response {
            Ok(credential) => Ok(HttpResponse::Ok().json(credential)),
            Err(err) => Ok(err.error_response()),
        }).responder()
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