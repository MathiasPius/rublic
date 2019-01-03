use actix::Handler;
use futures::Future;
use crate::errors::ServiceError;
use crate::cryptoutil::CryptoUtil;
use crate::database::messages::*;
use super::models::*;
use super::messages::*;
use super::AuthorizationManager;

impl Handler<AuthorizeUser> for AuthorizationManager {
    type Result = Result<Vec<Claim>, ServiceError>;

    fn handle(&mut self, msg: AuthorizeUser, _: &mut Self::Context) -> Self::Result {
        self.db.send(GetUserByName { friendly_name: msg.friendly_name.clone() }).flatten()
            .and_then(move |user|
                if CryptoUtil::check_key(&msg.password, &user.hashed_key) {
                    return Ok(user);
                } else {
                    return Err(ServiceError::Unauthorized);
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

    fn handle(&mut self, _msg: AuthorizeToken, _: &mut Self::Context) -> Self::Result {
        Err(ServiceError::InternalServerError)
    }
}