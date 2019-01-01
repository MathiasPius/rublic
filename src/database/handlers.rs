use actix::Handler;
use diesel::{MysqlConnection, prelude::*};
use crate::models::DbExecutor;
use crate::errors::ServiceError;
use super::models::*;
use super::messages::*;

impl Handler<GetDomainByFqdn> for DbExecutor {
    type Result = Result<Domain, ServiceError>;

    fn handle(&mut self, msg: GetDomainByFqdn, _: &mut Self::Context) -> Self::Result {
        use crate::schema::*;
        let conn: &MysqlConnection = &self.0.get().unwrap();

        let mut entry = domains::table
            .filter(domains::fqdn.eq(&msg.fqdn))
            .load::<Domain>(conn)
            .map_err(|_| ServiceError::InternalServerError)?;

        // There should never be more than one entry per fqdn.
        // Fail just in case there's a security issue here
        if entry.len() > 1 {
            return Err(ServiceError::InternalServerError);
        }

        Ok(entry.pop().ok_or(ServiceError::NotFound("domain with given fqdn not found".into()))?)
    }
}

impl Handler<GetGroupsByDomain> for DbExecutor {
    type Result = Result<Vec<Group>, ServiceError>;

    fn handle(&mut self, msg: GetGroupsByDomain, _: &mut Self::Context) -> Self::Result {
        use crate::schema::*;
        let conn: &MysqlConnection = &self.0.get().unwrap();

        let groups = domain_group_mappings::table
            .filter(domain_group_mappings::domain_id.eq(&msg.id))
            .inner_join(groups::table)
            .select((groups::id, groups::friendly_name, groups::permission))
            .load::<Group>(conn)
            .map_err(|_| ServiceError::InternalServerError)?;

        Ok(groups)
    }
}