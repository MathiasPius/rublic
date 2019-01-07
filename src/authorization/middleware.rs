use futures::Future;
use std::collections::HashSet;
use actix_web::{HttpRequest, Result, FromRequest, ResponseError};
use actix_web::middleware::{Middleware, Started};
use actix_web_httpauth::extractors::{
    basic::{BasicAuth, Config as BasicConfig},
    bearer::{BearerAuth, Config as BearerConfig}
};

use crate::errors::ServiceError;
use crate::app::AppState;
use super::messages::*;
use super::models::*;
use super::ValidateClaim;

fn expand_implied_claims(claim: &str) -> &[&str] {
    if claim == "private" {
        return &["public", "private"];
    } else if claim == "public" {
        return &["public"];
    }
    &[]
}

pub struct ClaimsProviderMiddleware { }

impl Middleware<AppState> for ClaimsProviderMiddleware {
    fn start(&self, req: &HttpRequest<AppState>) -> Result<Started> {
        if let Ok(basic) = BasicAuth::from_request(&req, &BasicConfig::default()) {
            // Attempt to authorize the user, and if it worked,
            // tag the request with their claims, otherwise return unauthorized
            let claims = req.state().authman
                .send(AuthorizeUser { 
                    friendly_name: basic.username().into(),
                    password: basic.password().unwrap_or("").into()
                }).flatten().wait()
                .and_then(|claims| {
                    let expanded_claims: HashSet<Claim> = claims.iter().map(move |claim|{
                        expand_implied_claims(&claim.permission).iter().map(move |permission| {
                            Claim {
                                subject: claim.subject.clone(),
                                permission: permission.to_string()
                            }
                        })
                    }).flatten().collect();

                    req.extensions_mut().insert(expanded_claims);
                    Ok(())
                });

            if let Err(e) = claims {
                return Ok(Started::Response(e.error_response()));
            }
        } else if let Ok(bearer) = BearerAuth::from_request(&req, &BearerConfig::default()) {
            let claims = req.state().authman
                .send(AuthorizeToken { 
                    token: bearer.token().into()
                }).flatten().wait()
                .and_then(|claims| {
                    req.extensions_mut().insert(claims);
                    Ok(())
                });

            if let Err(e) = claims {
                return Ok(Started::Response(e.error_response()));
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
        match req.validate_claims(&self.required_claims) {
            Ok(()) => Ok(Started::Done),
            Err(e) => Ok(Started::Response(ServiceError::from(e).error_response()))
        }
    }
}