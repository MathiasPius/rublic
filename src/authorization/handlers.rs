use std::env;
use chrono::Utc;
use actix::Handler;
use futures::Future;
use jwt::{encode, decode, Header, Algorithm, Validation};
use crate::errors::ServiceError;
use crate::cryptoutil::CryptoUtil;
use crate::database::messages::*;
use super::models::*;
use super::messages::*;
use super::AuthorizationManager;

lazy_static! {
    static ref JWT_HEADER: Header = Header::new(Algorithm::HS512);

    static ref JWT_VALIDATION: Validation = Validation {
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

    static ref ADMIN_PASSWORD: String = env::var("RUBLIC_ADMIN_PASSWORD")
        .expect("RUBLIC_ADMIN_PASSWORD was not defined!");
}

impl Handler<AuthorizeUser> for AuthorizationManager {
    type Result = Result<Vec<Claim>, ServiceError>;

    fn handle(&mut self, msg: AuthorizeUser, _: &mut Self::Context) -> Self::Result {

        // This is the only account that can actually make changes
        if msg.friendly_name == "admin" {
            if msg.password == ADMIN_PASSWORD.to_string() { 
                return Ok(vec![Claim { subject: "*".into(), permission: "*".into() }]);
            } else {
                return Err(ServiceError::Unauthorized);
            }
        }

        self.db.send(GetUserByName { friendly_name: msg.friendly_name.clone() }).flatten()
            .and_then(move |user|
                if CryptoUtil::check_key(&msg.password, &user.hashed_key) {
                    Ok(user)
                } else {
                    Err(ServiceError::Unauthorized)
                }
            ).and_then(move |user: crate::database::models::User| {
                self.db.send(GetUserPermissions{ id: user.id }).flatten()
                    .and_then(move |permissions: Vec<crate::database::models::DomainPermission>| {
                        Ok(permissions.into_iter().map(|permission|
                            Claim {
                                subject: permission.fqdn,
                                permission: permission.permission
                            }
                        ).collect())
                    })
            }).wait()
    }
}

impl Handler<AuthorizeToken> for AuthorizationManager {
    type Result = Result<Vec<Claim>, ServiceError>;

    fn handle(&mut self, msg: AuthorizeToken, _: &mut Self::Context) -> Self::Result {

        decode(&msg.token, JWT_SHARED_SECRET.as_ref(), &JWT_VALIDATION)
            .map_err(|e| e.into())
            .and_then(|token| Ok(token.claims))
    }
}

impl Handler<BuildTokenFromClaims> for AuthorizationManager {
    type Result = Result<String, ServiceError>;

    fn handle(&mut self, msg: BuildTokenFromClaims, _: &mut Self::Context) -> Self::Result {

        let now = Utc::now().timestamp();
        let token = Token {
            iat: now,
            nbf: now,
            exp: now + msg.lifetime.num_seconds(),
            aud: JWT_AUDIENCE.to_string(),
            iss: JWT_ISSUER.to_string(),
            claims: msg.claims
        };

        encode::<Token>(&JWT_HEADER, &token, JWT_SHARED_SECRET.as_ref()).map_err(|e| e.into())
    }
}