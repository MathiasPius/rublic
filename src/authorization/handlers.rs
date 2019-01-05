use chrono::Utc;
use actix::Handler;
use futures::Future;
use jwt::{encode, decode};
use crate::errors::ServiceError;
use crate::cryptoutil::CryptoUtil;
use crate::database::messages::*;
use crate::config::{ADMIN_PASSWORD, JWT_SHARED_SECRET, JWT_VALIDATION, JWT_ISSUER, JWT_AUDIENCE, JWT_HEADER};
use super::models::*;
use super::messages::*;
use super::AuthorizationManager;

impl Handler<AuthorizeUser> for AuthorizationManager {
    type Result = Result<Vec<Claim>, ServiceError>;

    fn handle(&mut self, msg: AuthorizeUser, _: &mut Self::Context) -> Self::Result {

        // This is the only account that can actually make changes
        if msg.friendly_name == "admin" {
            if msg.password == *ADMIN_PASSWORD { 
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

        decode::<Token>(&msg.token, JWT_SHARED_SECRET.as_ref(), &JWT_VALIDATION)
            .map_err(|e| e.into())
            .and_then(|token| Ok(token.claims.claims))
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

impl Handler<RefreshToken> for AuthorizationManager {
    type Result = Result<String, ServiceError>;

    fn handle(&mut self, msg: RefreshToken, _: &mut Self::Context) -> Self::Result {
        decode::<Token>(&msg.token, JWT_SHARED_SECRET.as_ref(), &JWT_VALIDATION)
            .map_err(|e| e.into())
            .and_then(|token| {
                let now = Utc::now().timestamp();
                let token = Token {
                    iat: now,
                    nbf: now,
                    exp: core::cmp::min(now + msg.lifetime.num_seconds(), token.claims.exp),
                    aud: JWT_AUDIENCE.to_string(),
                    iss: JWT_ISSUER.to_string(),
                    claims: token.claims.claims
                };

                encode::<Token>(&JWT_HEADER, &token, JWT_SHARED_SECRET.as_ref()).map_err(|e| e.into())
            })
    }
}