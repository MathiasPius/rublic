pub mod models;
pub mod messages;
pub mod middleware;
pub mod errors;
mod handlers;

use actix::{Actor, Addr, Context};
use actix_web::{HttpRequest, Result};

use crate::database::DbExecutor;
use crate::app::AppState;
use self::models::*;
use self::errors::Error;
pub use self::middleware::{ClaimsCheckerMiddleware, ClaimsProviderMiddleware};


pub struct AuthorizationManager {
    pub db: Addr<DbExecutor>
}

impl Actor for AuthorizationManager {
    type Context = Context<Self>;
}

pub trait ValidateClaim {
    fn validate_claims(&self, required_claims: &[Claim]) -> Result<(), Error>;
}

impl<S> ValidateClaim for HttpRequest<S> {
    fn validate_claims(&self, required_claims: &[Claim]) -> Result<(), Error> {
        if let Some(actual_claims) = self.extensions().get::<Vec<Claim>>() {

            // Short-circuit in the case of the administrator
            if actual_claims.contains(&Claim { subject: "*".into(), permission: "*".into()}) {
                return Ok(())
            }

            let params = self.match_info();

            for required_claim in required_claims {
                // Required claims are stated in the form of (parameter_name, permission)
                // where the parameter is extracted from the URL using the parameter_name
                // and checked against  the actual claims attached to the request.
                //
                // The endpoint /api/domains/{fqdn}/ might validate the {fqdn} by
                // adding the following middleware to the path:
                //
                //     scope.middleware(authorize(&[("fqdn", "public")]))
                //
                // The middleware will then expand the fqdn from the parameter,
                // and verify that it exists in the claims attached to the request.
                // For instance, accessing /api/domains/example.com/ would check if
                //
                //      example.com:public 
                // 
                // ... was present in the HttpRequest's claims.

                match params.get(&required_claim.subject) {
                    Some(subject) => {
                        let resolved_claim = Claim { 
                            subject: subject.into(), 
                            permission: required_claim.permission.clone()
                        };

                        if actual_claims.contains(&resolved_claim) {
                            continue;
                        }

                        return Err(Error::NotAuthorized(resolved_claim.permission, resolved_claim.subject));
                    },
                    None => return Err(Error::MissingResourceParameter(required_claim.subject.clone()))
                }
            }
        } else {
            return Err(Error::NotAuthenticated)
        }
        
        Ok(())
    }
}

pub trait ResourceAuthorization {
    fn authorize_resource(self, resource: &str, permission: &str) -> Self;
}

impl ResourceAuthorization for actix_web::Scope<AppState> {
    fn authorize_resource(self, resource: &str, permission: &str) -> Self {
        self.middleware(ClaimsCheckerMiddleware {
            required_claims: vec!(Claim {
                subject: resource.to_string(),
                permission: permission.to_string()
            })
        })
    }
}