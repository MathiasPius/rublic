pub mod models;
pub mod messages;
mod handlers;

use std::env;
use chrono::Duration;
use futures::Future;
use actix::{Actor, Addr, Context};
use actix_web::{HttpRequest, HttpResponse, Result, FromRequest};
use actix_web::middleware::{Middleware, Started};
use actix_web_httpauth::extractors::{
    basic::{BasicAuth, Config as BasicConfig},
    bearer::{BearerAuth, Config as BearerConfig}
};
use jwt::{Header, Algorithm, Validation};

use crate::database::DbExecutor;
use crate::app::AppState;
use self::messages::*;
use self::models::*;


lazy_static! {
    pub static ref JWT_HEADER: Header = Header::new(Algorithm::HS512);

    pub static ref JWT_VALIDATION: Validation = Validation {
        leeway: 5,
        validate_exp: true,
        validate_iat: true,
        validate_nbf: true,
        aud: Some(JWT_AUDIENCE.to_string().into()),
        iss: Some(JWT_ISSUER.to_string().into()),
        sub: None,
        algorithms: vec![Algorithm::HS512]
    };

    static ref JWT_SHARED_SECRET: String = std::env::var("RUBLIC_SHARED_SECRET")
        .expect("RUBLIC_SHARED_SECRET environment variable is not defined!");

    static ref JWT_AUDIENCE: String = env::var("RUBLIC_JWT_AUDIENCE")
        .unwrap_or("rublic-audience".into());

    static ref JWT_ISSUER: String = env::var("RUBLIC_JWT_ISSUER")
        .unwrap_or("rublic-issuer".into());

    pub static ref JWT_ACCESS_LIFETIME: Duration = Duration::hours(1);
    pub static ref JWT_REFRESH_LIFETIME: Duration = Duration::days(30);

    static ref ADMIN_PASSWORD: String = env::var("RUBLIC_ADMIN_PASSWORD")
        .expect("RUBLIC_ADMIN_PASSWORD was not defined!");
}


pub trait ValidateClaim {
    fn validate_claims(&self, required_claims: &[Claim]) -> bool;
}

fn expand_permissions(claim: &str) -> Vec<&str> {
    if claim == "public" {
        return vec!["public", "private"];
    }

    if claim == "private" {
        return vec!["private"];
    }

    return vec![];
}

impl<S> ValidateClaim for HttpRequest<S> {
    fn validate_claims(&self, required_claims: &[Claim]) -> bool {
        if let Some(actual_claims) = self.extensions().get::<Vec<Claim>>() {

            // Short-circuit in the case of the administrator
            if actual_claims.contains(&Claim { subject: "*".into(), permission: "*".into()}) {
                return true;
            }

            let params = self.match_info();
            
            for required_claim in required_claims.iter() {
                if let Some(claim) = params.get(&required_claim.subject) {
                    if !expand_permissions(&required_claim.permission).into_iter().any(|permission|
                        actual_claims.contains(&Claim { 
                            subject: claim.into(), 
                            permission: permission.into() 
                        })
                    ) {
                        println!("user failed claims check for: {:?}, only had: {:?}", &required_claim, actual_claims);
                        return false;
                    }
                } else {
                    println!("authorizing against unknown parameter: {}", &required_claim.subject);
                    // If the parameter we're trying to authorize against isn't in the path,
                    // err on the side of caution and abort
                    return false;
                }
            }
        } else {
            // If we get to this point, it means that no Vec<Claim> extension has been
            // registered on the request object, meaning the user has not been authorized.
            // Since this Middleware has been triggered, *some* authorization was intended
            return false;
        }
        
        true
    }
}

pub struct AuthorizationManager {
    pub db: Addr<DbExecutor>
}

impl Actor for AuthorizationManager {
    type Context = Context<Self>;
}


pub struct ClaimsProviderMiddleware { }

impl Middleware<AppState> for ClaimsProviderMiddleware {
    fn start(&self, req: &HttpRequest<AppState>) -> Result<Started> {
;
        if let Ok(basic) = BasicAuth::from_request(&req, &BasicConfig::default()) {
            // Attempt to authorize the user, and if it worked,
            // tag the request with their claims, otherwise return unauthorized
            let claims = req.state().authman
                .send(AuthorizeUser { 
                    friendly_name: basic.username().into(),
                    password: basic.password().unwrap_or("").into()
                }).flatten().wait()
                .and_then(|claims| {
                    req.extensions_mut().insert(claims);
                    Ok(())
                });

            if claims.is_err() {
                return Ok(Started::Response(HttpResponse::Unauthorized().finish()));
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

            if claims.is_err() {
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
        if !req.validate_claims(&self.required_claims) {
            Ok(Started::Response(HttpResponse::Unauthorized().finish()))
        } else {
            Ok(Started::Done)
        }
    }
}

pub fn authorize(claims: &[(&str, &str)]) -> ClaimsCheckerMiddleware {
    ClaimsCheckerMiddleware {
        required_claims: claims.iter().map(|claims| {
            Claim {
                subject: claims.0.into(),
                permission: claims.1.into()
            }
        }).collect()
    }
}