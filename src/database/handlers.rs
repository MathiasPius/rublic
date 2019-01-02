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

        domain_group_mappings::table
            .filter(domain_group_mappings::domain_id.eq(&msg.id))
            .inner_join(groups::table)
            .select((groups::id, groups::friendly_name, groups::permission))
            .load::<Group>(conn)
            .map_err(|e| e.into())
    }
}

impl Handler<GetGroupsByUser> for DbExecutor {
    type Result = Result<Vec<Group>, ServiceError>;

    fn handle(&mut self, msg: GetGroupsByUser, _: &mut Self::Context) -> Self::Result {
        use crate::schema::*;
        let conn: &MysqlConnection = &self.0.get().unwrap();

        user_group_mappings::table
            .filter(user_group_mappings::user_id.eq(&msg.id))
            .inner_join(groups::table)
            .select((groups::id, groups::friendly_name, groups::permission))
            .load::<Group>(conn)
            .map_err(|e| e.into())
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

impl Handler<SetGroupUsers> for DbExecutor {
    type Result = Result<usize, ServiceError>;

    fn handle(&mut self, msg: SetGroupUsers, _: &mut Self::Context) -> Self::Result {
        use crate::schema::*;
        let conn: &MysqlConnection = &self.0.get().unwrap();

        let group_id = msg.group_id.clone();

        let mappings: Vec<UserGroupMapping> = msg.user_ids.into_iter().map(|id| {
            UserGroupMapping {
                user_id: id.clone(),
                group_id: group_id.clone()
            }
        }).collect();

        diesel::delete(user_group_mappings::table)
            .filter(user_group_mappings::group_id.eq(&msg.group_id))
            .execute(conn)?;

        diesel::insert_into(user_group_mappings::table)
            .values(&mappings)
            .execute(conn)?;

        Ok(mappings.len())
    }
}

impl Handler<SetGroupDomains> for DbExecutor {
    type Result = Result<usize, ServiceError>;

    fn handle(&mut self, msg: SetGroupDomains, _: &mut Self::Context) -> Self::Result {
        use crate::schema::*;
        let conn: &MysqlConnection = &self.0.get().unwrap();

        let group_id = msg.group_id.clone();

        let mappings: Vec<DomainGroupMapping> = msg.domain_ids.into_iter().map(|id| {
            DomainGroupMapping {
                domain_id: id.clone(),
                group_id: group_id.clone()
            }
        }).collect();

        diesel::delete(domain_group_mappings::table)
            .filter(domain_group_mappings::group_id.eq(&msg.group_id))
            .execute(conn)?;

        diesel::insert_into(domain_group_mappings::table)
            .values(&mappings)
            .execute(conn)?;

        Ok(mappings.len())
    }
}

impl Handler<GetUsersByGroup> for DbExecutor {
    type Result = Result<Vec<User>, ServiceError>;

    fn handle(&mut self, msg: GetUsersByGroup, _: &mut Self::Context) -> Self::Result {
        use crate::schema::*;
        let conn: &MysqlConnection = &self.0.get().unwrap();

        user_group_mappings::table
            .filter(user_group_mappings::group_id.eq(&msg.id))
            .inner_join(users::table)
            .select((users::id, users::friendly_name, users::hashed_key))
            .load::<User>(conn)
            .map_err(|e| e.into())
    }
}

impl Handler<GetDomainsByGroup> for DbExecutor {
    type Result = Result<Vec<Domain>, ServiceError>;

    fn handle(&mut self, msg: GetDomainsByGroup, _: &mut Self::Context) -> Self::Result {
        use crate::schema::*;
        let conn: &MysqlConnection = &self.0.get().unwrap();

        domain_group_mappings::table
            .filter(domain_group_mappings::group_id.eq(&msg.id))
            .inner_join(domains::table)
            .select((domains::id, domains::fqdn, domains::hashed_fqdn))
            .load::<Domain>(conn)
            .map_err(|e| e.into())
    }
}

impl Handler<GetGroups> for DbExecutor {
    type Result = Result<Vec<Group>, ServiceError>;

    fn handle(&mut self, _: GetGroups, _: &mut Self::Context) -> Self::Result {
        use crate::schema::*;
        let conn: &MysqlConnection = &self.0.get().unwrap();

        groups::table
            .load::<Group>(conn)
            .map_err(|e| e.into())
    }
}