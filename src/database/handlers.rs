use actix::Handler;
use diesel::{MysqlConnection, prelude::*};
use crate::models::DbExecutor;
use crate::errors::ServiceError;
use crate::cryptoutil::CryptoUtil;
use super::models::*;
use super::messages::*;

impl Handler<CreateDomain> for DbExecutor {
    type Result = Result<Domain, ServiceError>;

    fn handle(&mut self, msg: CreateDomain, _: &mut Self::Context) -> Self::Result {
        use crate::schema::*;
        let conn: &MysqlConnection = &self.0.get().unwrap();

        let new_domain = Domain {
            id: CryptoUtil::generate_uuid(),
            hashed_fqdn: CryptoUtil::hash_string(&msg.fqdn),
            fqdn: msg.fqdn,
        };

        diesel::insert_into(domains::table)
            .values(&new_domain)
            .execute(conn)?;

        Ok(new_domain)
    }
}

impl Handler<GetDomainByFqdn> for DbExecutor {
    type Result = Result<Domain, ServiceError>;

    fn handle(&mut self, msg: GetDomainByFqdn, _: &mut Self::Context) -> Self::Result {
        use crate::schema::*;
        let conn: &MysqlConnection = &self.0.get().unwrap();

        let mut entry = domains::table
            .filter(domains::fqdn.eq(&msg.fqdn))
            .load::<Domain>(conn)?;

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
            .load::<Group>(conn)?;

        Ok(groups)
    }
}

impl Handler<GetGroupsByUser> for DbExecutor {
    type Result = Result<Vec<Group>, ServiceError>;

    fn handle(&mut self, msg: GetGroupsByUser, _: &mut Self::Context) -> Self::Result {
        use crate::schema::*;
        let conn: &MysqlConnection = &self.0.get().unwrap();

        let groups = user_group_mappings::table
            .filter(user_group_mappings::user_id.eq(&msg.id))
            .inner_join(groups::table)
            .select((groups::id, groups::friendly_name, groups::permission))
            .load::<Group>(conn)?;

        Ok(groups)
    }
}


impl Handler<CreateUser> for DbExecutor {
    type Result = Result<User, ServiceError>;

    fn handle(&mut self, msg: CreateUser, _: &mut Self::Context) -> Self::Result {
        use crate::schema::*;
        let conn: &MysqlConnection = &self.0.get().unwrap();

        let new_user = User {
            id: CryptoUtil::generate_uuid(),
            friendly_name: msg.friendly_name,
            hashed_key: msg.hashed_key
        };

        diesel::insert_into(users::table)
            .values(&new_user)
            .execute(conn)?;

        Ok(new_user)
    }
}

impl Handler<GetUserByName> for DbExecutor {
    type Result = Result<User, ServiceError>;

    fn handle(&mut self, msg: GetUserByName, _: &mut Self::Context) -> Self::Result {
        use crate::schema::*;
        let conn: &MysqlConnection = &self.0.get().unwrap();

        let mut user = users::table
            .filter(users::friendly_name.eq(&msg.friendly_name))
            .limit(1)
            .load::<User>(conn)?;

        user.pop().ok_or(ServiceError::NotFound("user with that name not found".to_string()))
    }
}

impl Handler<GetUser> for DbExecutor {
    type Result = Result<User, ServiceError>;

    fn handle(&mut self, msg: GetUser, _: &mut Self::Context) -> Self::Result {
        use crate::schema::*;
        let conn: &MysqlConnection = &self.0.get().unwrap();

        let mut user = users::table
            .filter(users::id.eq(&msg.id))
            .limit(1)
            .load::<User>(conn)?;

        user.pop().ok_or(ServiceError::NotFound("user with that id not found".to_string()))
    }
}

impl Handler<CreateGroup> for DbExecutor {
    type Result = Result<Group, ServiceError>;

    fn handle(&mut self, msg: CreateGroup, _: &mut Self::Context) -> Self::Result {
        use crate::schema::*;
        let conn: &MysqlConnection = &self.0.get().unwrap();

        let new_group = Group {
            id: CryptoUtil::generate_uuid(),
            friendly_name: msg.friendly_name,
            permission: "read".into()
        };

        diesel::insert_into(groups::table)
            .values(&new_group)
            .execute(conn)?;

        Ok(new_group)
    }
}

impl Handler<GetGroup> for DbExecutor {
    type Result = Result<Group, ServiceError>;

    fn handle(&mut self, msg: GetGroup, _: &mut Self::Context) -> Self::Result {
        use crate::schema::*;
        let conn: &MysqlConnection = &self.0.get().unwrap();

        let mut group = groups::table
            .filter(groups::id.eq(&msg.id))
            .limit(1)
            .load::<Group>(conn)?;

        group.pop().ok_or(ServiceError::NotFound("group with that id not found".to_string()))
    }
}