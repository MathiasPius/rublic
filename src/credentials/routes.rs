use actix_web::{AsyncResponder, FutureResponse, HttpResponse, Json, ResponseError, State, http::Method, Scope, Path};
use futures::future::Future;

use crate::cryptutil::CryptUtil;
use crate::app::AppState;
use crate::credentials::models::{internal::*, external::*};

pub fn register(router: Scope<AppState>) -> Scope<AppState> {
    router
        // Credentials
        .resource("/credentials", |r| {
            r.method(Method::POST).with(new_access_credential);
            r.method(Method::GET).with(get_all_access_credentials);
        })
        .resource("/credentials/{access_credential_id}", |r| {
            r.method(Method::GET).with(get_expanded_access_credential);
        })

        // Access Groups
        .resource("/accessgroups", |r| {
            r.method(Method::GET).with(get_all_access_groups);
        })
        .resource("/accessgroups/{access_group_id}", |r| {
            r.method(Method::GET).with(get_expanded_access_group)
        })
        /*
        .resource("/certificates/{certificate_id}", |r| {
            r.method(Method::GET).with(get_certificate_by_id);
        })
        */
}

fn new_access_credential((request, state): (Json<CreateAccessCredentialRequest>, State<AppState>))
    -> FutureResponse<HttpResponse> {
    let key = CryptUtil::generate_key();
    let hashed_key = CryptUtil::hash_key(&key);

    state
        .db
        .send(CreateAccessCredential {
            friendly_name: request.into_inner().friendly_name,
            hashed_key
        })
        .from_err()
        .and_then(|db_response| match db_response {
            // Re-pack the AccessCredential with the unhashed secret key.
            // This is not saved anywhere and only exists in this moment as it is returned
            Ok(credential) => Ok(HttpResponse::Ok().json(NewlyCreatedAccessCredential {
                friendly_name: credential.friendly_name,
                secret_key: key
            })),
            Err(err) => Ok(err.error_response()),
        }).responder()
}


fn get_all_access_credentials(state: State<AppState>) -> FutureResponse<HttpResponse>
{
    state
        .db
        .send(GetAllAccessCredentials {})
        .from_err()
        .and_then(|db_response| match db_response {
            Ok(credentials) => Ok(HttpResponse::Ok().json(credentials)),
            Err(err) => Ok(err.error_response()),
        }).responder()
}

fn get_expanded_access_credential((access_credential_id, state): (Path<String>, State<AppState>)) 
    -> FutureResponse<HttpResponse> {

    state
        .db
        .send(GetExpandedAccessCredential { id: access_credential_id.into_inner() })
        .from_err()
        .and_then(|db_response| match db_response {
            Ok(credential) => Ok(HttpResponse::Ok().json(credential)),
            Err(err) => Ok(err.error_response()),
        }).responder()
}

fn get_expanded_access_group((access_group_id, state): (Path<String>, State<AppState>)) 
    -> FutureResponse<HttpResponse> {

    state
        .db
        .send(GetExpandedAccessGroup { id: access_group_id.into_inner() })
        .from_err()
        .and_then(|db_response| match db_response {
            Ok(group) => Ok(HttpResponse::Ok().json(group)),
            Err(err) => Ok(err.error_response()),
        }).responder()
}

fn get_all_access_groups(state: State<AppState>) -> FutureResponse<HttpResponse>
{
    state
        .db
        .send(GetAllAccessGroups {})
        .from_err()
        .and_then(|db_response| match db_response {
            Ok(groups) => Ok(HttpResponse::Ok().json(groups)),
            Err(err) => Ok(err.error_response()),
        }).responder()
}