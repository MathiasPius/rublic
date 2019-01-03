pub mod models;
pub mod messages;
mod handlers;

use futures::Future;
use actix::{Actor, Addr, Context};
use actix_web::{HttpRequest, HttpResponse, Result, FromRequest};
use actix_web::middleware::{Middleware, Started};
use actix_web_httpauth::extractors::{
    basic::{BasicAuth, Config as BasicConfig},
    //bearer::{BearerAuth, Config as BearerConfig}
};

use crate::database::DbExecutor;
use crate::app::AppState;
use self::messages::*;
use self::models::*;

pub struct AuthorizationManager {
    pub db: Addr<DbExecutor>
}

impl Actor for AuthorizationManager {
    type Context = Context<Self>;
}


pub struct ClaimsProviderMiddleware { }

impl Middleware<AppState> for ClaimsProviderMiddleware {
    fn start(&self, req: &HttpRequest<AppState>) -> Result<Started> {

        let config = BasicConfig::default();
        if let Ok(basic) = BasicAuth::from_request(&req, &config)
        {
            // Attempt to authorize the user, and if it worked,
            // tag the request with their claims, otherwise return unauthorized
            if !req.state().authman.send(AuthorizeUser { 
                friendly_name: basic.username().into(),
                password: basic.password().unwrap_or("").into()
             }).flatten().wait()
             .and_then(|claims| {
                 req.extensions_mut().insert(claims);
                 Ok(())
             }).is_ok() {
                return Ok(Started::Response(HttpResponse::Unauthorized().finish()));
             }
        }

        Ok(Started::Done)
    }
}

pub struct ClaimsCheckerMiddleware { 
    pub required_claims: Vec<Claim>
}

impl Middleware<AppState> for ClaimsCheckerMiddleware {
    fn start(&self, req: &HttpRequest<AppState>) -> Result<Started> {
        if let Some(actual_claims) = req.extensions().get::<Vec<Claim>>() {
            let params = req.match_info();

            // Make sure user has all required claims
            for required_claim in &self.required_claims {
                if let Some(claim) = params.get(&required_claim.subject) {
                    if !actual_claims.contains(&Claim { subject: claim.into(), permission: required_claim.permission.clone() }) {
                        return Ok(Started::Response(HttpResponse::Unauthorized().finish()));
                    }
                } else {
                    // If the parameter we're trying to authorize against isn't in the path,
                    // err on the side of caution and abort
                    return Ok(Started::Response(HttpResponse::Unauthorized().finish()));
                }
            }
        } else {
            // If we get to this poinrt, it means that no Vec<Claim> extension has been
            // registered on the request object, meaning the user has not been authorized.
            // Since this Middleware has been triggered, *some* authorization was intended
            return Ok(Started::Response(HttpResponse::Unauthorized().finish()));
        }

        // If all the required_claims have been found in the actual_claims,
        // we've successfully been cleared
        Ok(Started::Done)
    }
}

pub fn authorize(claims: Vec<(&str, &str)>) -> ClaimsCheckerMiddleware {
    ClaimsCheckerMiddleware {
        required_claims: claims.into_iter().map(|claims| {
            Claim {
                subject: claims.0.into(),
                permission: claims.1.into()
            }
        }).collect()
    }
}