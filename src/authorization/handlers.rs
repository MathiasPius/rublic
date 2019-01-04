use std::env;
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

        // This is the only account that can actually make changes
        if msg.friendly_name == "admin" {
            let admin_password = env::var("RUBLIC_ADMIN_PASSWORD")
                .expect("No administrator password was set!");

            if msg.password == admin_password { 
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

    fn handle(&mut self, _msg: AuthorizeToken, _: &mut Self::Context) -> Self::Result {
        Err(ServiceError::InternalServerError)
    }
}