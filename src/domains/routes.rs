use actix_web::{AsyncResponder, FutureResponse, HttpResponse, Json, ResponseError, State, http::Method, Scope, Path};
use futures::future::Future;

use crate::app::AppState;
use crate::domains::models::{NewCertificateEntry, GetCertificateById};

pub fn register(router: Scope<AppState>) -> Scope<AppState> {
    router
        .resource("/certificates/", |r| {
            r.method(Method::POST).with(new_certificate_entry);
        })
        .resource("/certificates/{certificate_id}", |r| {
            r.method(Method::GET).with(get_certificate_by_id);
        })
}

fn new_certificate_entry((certificate_entry, state): (Json<NewCertificateEntry>, State<AppState>))
    -> FutureResponse<HttpResponse> {
    state
        .db
        .send(certificate_entry.into_inner())
        .from_err()
        .and_then(|db_response| match db_response {
            Ok(certificate) => Ok(HttpResponse::Ok().json(certificate)),
            Err(err) => Ok(err.error_response()),
        }).responder()
}

fn get_certificate_by_id((certificate_id, state): (Path<String>, State<AppState>))
    -> FutureResponse<HttpResponse> {
    state
        .db
        .send(GetCertificateById { id: certificate_id.into_inner() })
        .from_err()
        .and_then(|db_response| match db_response {
            Ok(certificate) => Ok(HttpResponse::Ok().json(certificate)),
            Err(err) => Ok(err.error_response()),
        }).responder()
}