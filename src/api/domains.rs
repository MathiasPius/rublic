use actix::Addr;
use actix_web::{State, http::Method, Scope, HttpRequest, HttpResponse, FutureResponse, Path, AsyncResponder};
use futures::future::Future;
use crate::app::AppState;
use crate::errors::ServiceError;
use crate::database::DbExecutor;
use crate::database::messages::*;
use crate::certman::messages::*;
use crate::certman::CertificateManager;
use crate::authorization::{ValidateClaim, authorize};
use crate::authorization::models::*;
use super::into_api_response;
use super::models::*;

pub fn register(router: Scope<AppState>) -> Scope<AppState> {
    router
        .nested("/{fqdn}", |entry| {
            entry.middleware(authorize(&[("fqdn", "public")]))
            .resource("", |r| {
                r.method(Method::GET).with_async(api_get_domain);
                r.method(Method::POST).with_async(api_create_domain);
            })
            .nested("/certs", |certs| {
                certs.nested("/latest", |latest| {
                    latest.resource("/{filename}", |r| {
                        r.method(Method::GET).with_async(api_get_domain_latest_certificate);
                    })
                    .resource("", |r| {
                        r.method(Method::GET).with_async(api_get_domain_latest_certificates_version);
                    })
                })
                .nested("/{version}", |version| {
                    version.resource("/{filename}", |r| {
                        r.method(Method::GET).with_async(api_get_domain_certificate);
                    })
                    .resource("", |r| {
                        r.method(Method::GET).with_async(api_get_domain_certificates_version);
                    })
                })
                .resource("", |r| {
                    r.method(Method::GET).with_async(api_get_domain_certificates);
                })
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
            groups: None,
            latest_certs: None
        }))
        .from_err()
    )
}

fn api_get_domain((fqdn, state): (Path<String>, State<AppState>))
    -> FutureResponse<HttpResponse> {
  
    into_api_response(get_domain_by_fqdn(state.db.clone(), fqdn.to_string()))
}

fn api_get_domain_certificates((fqdn, state): (Path<String>, State<AppState>)) 
    -> FutureResponse<HttpResponse> {

    into_api_response(state.db.send(GetDomainByFqdn { fqdn: fqdn.into_inner() }).flatten()
        .and_then(move |domain| {
            get_domains_certificates(state.db.clone(), domain.id)
        }))
}

fn api_get_domain_certificate((path, state, req): (Path<(String, i32, String)>, State<AppState>, HttpRequest<AppState>))
    -> FutureResponse<HttpResponse> {

    let (fqdn, version, friendly_name) = path.into_inner();
    
    get_domain_certificate(state.db.clone(), state.certman.clone(), (fqdn, Some(version), friendly_name))
        // If the returned certificate is a private key, make sure the user 
        // is allowed to see them, before transmitting them
        .and_then(move |result| {
            if result.is_private && !req.validate_claims(&[Claim { subject: "fqdn".into(), permission: "private".into()}]) {
                return Err(ServiceError::Unauthorized);
            }

            Ok(result)
        })
        .and_then(|result| {
            Ok(HttpResponse::Ok()
                .content_type("application/x-pem-file")
                .body(result.raw_data))
        })
        .from_err()
        .responder()
}

fn api_get_domain_latest_certificate((path, state, req): (Path<(String, String)>, State<AppState>, HttpRequest<AppState>))
    -> FutureResponse<HttpResponse> {

    let (fqdn, friendly_name) = path.into_inner();
    
    get_domain_certificate(state.db.clone(), state.certman.clone(), (fqdn, None, friendly_name))
        // If the returned certificate is a private key, make sure the user 
        // is allowed to see them, before transmitting them
        .and_then(move |result| {
            if result.is_private && !req.validate_claims(&[Claim { subject: "fqdn".into(), permission: "private".into()}]) {
                return Err(ServiceError::Unauthorized);
            }

            Ok(result)
        })
        .and_then(|result| {
            Ok(HttpResponse::Ok()
                .content_type("application/x-pem-file")
                .body(result.raw_data))
        })
        .from_err()
        .responder()
}

fn get_domain_certificate(db: Addr<DbExecutor>, certman: Addr<CertificateManager>, (fqdn, version, friendly_name): (String, Option<i32>, String))
    -> impl Future<Item = RawCertificate, Error = ServiceError>
{
    db.send(GetDomainByFqdn{ fqdn }).flatten()
        .map_err(|_| ServiceError::InternalServerError)
        .and_then(move |domain|
            db.send(GetCertificate { 
                domain_id: domain.id, 
                id: version,
                friendly_name
            }).flatten()
            .map_err(|_| ServiceError::InternalServerError)
            .and_then(move |cert| {
                certman.send(GetCertificateByPath{ path: cert.path.clone() }).flatten()
                    .map_err(|_| ServiceError::InternalServerError)
                    .and_then(move |file| {
                        Ok(RawCertificate {
                            raw_data: file.raw_data,
                            is_private: cert.is_private
                        })
                    }).map_err(|_| ServiceError::InternalServerError)
            })
        )
}

fn api_get_domain_certificates_version((path, state): (Path<(String, i32)>, State<AppState>))
    -> FutureResponse<HttpResponse> {

    let (fqdn, version) = path.into_inner();

    into_api_response(
        state.db.send(GetDomainByFqdn{ fqdn }).flatten()
            .and_then(move |domain|
                get_domain_certificates_version(state.db.clone(), (domain.id, Some(version)))
            )
    )
}

fn api_get_domain_latest_certificates_version((fqdn, state): (Path<String>, State<AppState>))
    -> FutureResponse<HttpResponse> {

    into_api_response(
        state.db.send(GetDomainByFqdn{ fqdn: fqdn.into_inner() }).flatten()
            .and_then(move |domain|
                get_domain_certificates_version(state.db.clone(), (domain.id, None))
            )
    )
}

fn get_domain_certificates_version(db: Addr<DbExecutor>, (domain_id, version): (String, Option<i32>))
    -> impl Future<Item = Vec<Certificate>, Error = ServiceError> {

    db.send(GetCertificatesByDomainAndId { 
            domain_id, 
            id: version
        }).flatten()
        .and_then(move |certificates| -> Result<Vec<Certificate>, ServiceError> {
            Ok(certificates.into_iter().map(|cert| Certificate {
                version: cert.id,
                friendly_name: cert.friendly_name,
                is_private: cert.is_private,
                not_before: cert.not_before,
                not_after: cert.not_after
            }).collect())
        })
}

fn get_domains_groups(db: Addr<DbExecutor>, id: String) 
    -> impl Future<Item = Vec<PluggableGroup>, Error = ServiceError> {
    db.send(GetGroupsByDomain { id }).flatten()
        .and_then(|groups| 
            Ok(groups.into_iter().map(|group| PluggableGroup {
                id: group.id,
                friendly_name: group.friendly_name,
                domains: None,
                users: None
            }).collect())
        )
}

fn get_domains_certificates(db: Addr<DbExecutor>, id: String) 
    -> impl Future<Item = Vec<Certificate>, Error = ServiceError> {
    db.send(GetCertificatesByDomain { id }).flatten()
        .and_then(|certificates|
            Ok(certificates.into_iter().map(|cert| {
                Certificate {
                    version: cert.id,
                    friendly_name: cert.friendly_name,
                    is_private: cert.is_private,
                    not_after: cert.not_after,
                    not_before: cert.not_before
                }
            }).collect())
        )
}

fn get_domain_by_fqdn(db: Addr<DbExecutor>, fqdn: String)
    -> impl Future<Item = PluggableDomain, Error = ServiceError> {
    db.send(GetDomainByFqdn { fqdn }).flatten()
        .and_then(move |domain| 
            get_domains_groups(db.clone(), domain.id.clone())
                .join(get_domain_certificates_version(db.clone(), (domain.id.clone(), None)))
                .and_then(|(groups, certificates)| Ok(PluggableDomain {
                    id: domain.id.clone(),
                    fqdn: domain.fqdn,
                    groups: Some(groups),
                    latest_certs: Some(certificates)
                }))
        )
}