use diesel::{prelude::*};
use crate::database::DbExecutor;
use crate::errors::ServiceError;
use crate::cryptoutil::CryptoUtil;
use super::models::*;
use super::messages::*;

// Simple rule for wrapping Handler implementations
macro_rules! impl_handler {   
    ($message:ident ( $conn:ident, $msg:ident, $ctx:ident ) for $actor:ty $blk:block) => {
        impl actix::Handler<$message> for $actor {
            type Result = <$message as actix::Message>::Result;

            fn handle(&mut self, msg: $message, $ctx: &mut Self::Context) -> Self::Result {
                use crate::schema::*;
                let conn: &diesel::mysql::MysqlConnection = &self.0.get().unwrap();

                let inner = |$msg: $message, $conn: &diesel::mysql::MysqlConnection| $blk;

                inner(msg, conn)
            }
        }
    };
    ($message:ident ( $conn:ident, $msg:ident ) for $actor:ty $blk:block) => {
        impl_handler! ($message( $conn, $msg, _ctx) for $actor $blk);
    };
    ($message:ident ( $conn:ident ) for $actor:ty $blk:block) => {
        impl_handler! ($message( $conn, _msg, _ctx) for $actor $blk);
    }
}


impl_handler! (CreateDomain(conn, msg) for DbExecutor {
    let new_domain = Domain {
        id: CryptoUtil::generate_uuid(),
        hashed_fqdn: CryptoUtil::hash_string(&msg.fqdn),
        fqdn: msg.fqdn,
    };

    diesel::insert_into(domains::table)
        .values(&new_domain)
        .execute(conn)?;

    Ok(new_domain)
});

impl_handler! (DeleteDomain(conn, msg) for DbExecutor {
    diesel::delete(domains::table)
        .filter(domains::fqdn.eq(&msg.fqdn))
        .execute(conn)
        .map_err(|e| e.into())
});

impl_handler! (GetDomainByFqdn(conn, msg) for DbExecutor {
    let mut entry = domains::table
        .filter(domains::fqdn.eq(&msg.fqdn))
        .load::<Domain>(conn)?;

    // There should never be more than one entry per fqdn.
    // Fail just in case there's a security issue here
    if entry.len() > 1 {
        return Err(ServiceError::InternalServerError);
    }

    Ok(entry.pop().ok_or(ServiceError::NotFound("domain with given fqdn not found".into()))?)
});


impl_handler! (GetGroupsByDomain(conn, msg) for DbExecutor {
    domain_group_mappings::table
        .filter(domain_group_mappings::domain_id.eq(&msg.id))
        .inner_join(groups::table)
        .select((groups::id, groups::friendly_name, groups::permission))
        .load::<Group>(conn)
        .map_err(|e| e.into())
});

impl_handler! (GetGroupsByUser(conn, msg) for DbExecutor {
    user_group_mappings::table
        .filter(user_group_mappings::user_id.eq(&msg.id))
        .inner_join(groups::table)
        .select((groups::id, groups::friendly_name, groups::permission))
        .load::<Group>(conn)
        .map_err(|e| e.into())    
});

impl_handler! (CreateUser(conn, msg) for DbExecutor {
    if msg.friendly_name == "admin" {
        return Err(ServiceError::Conflict("admin username is reserved".into()));
    }

    let new_user = User {
        id: CryptoUtil::generate_uuid(),
        friendly_name: msg.friendly_name,
        hashed_key: msg.hashed_key
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(conn)?;

    Ok(new_user)
});

impl_handler! (GetUserByName(conn, msg) for DbExecutor {
    let mut user = users::table
        .filter(users::friendly_name.eq(&msg.friendly_name))
        .limit(1)
        .load::<User>(conn)?;

    user.pop().ok_or(ServiceError::NotFound("user with that name not found".to_string()))
});

impl_handler! (GetUser(conn, msg) for DbExecutor {
    let mut user = users::table
        .filter(users::id.eq(&msg.id))
        .limit(1)
        .load::<User>(conn)?;

    user.pop().ok_or(ServiceError::NotFound("user with that id not found".to_string()))
});

impl_handler! (GetUserPermissions(conn, msg) for DbExecutor{
    user_group_mappings::table
        .filter(user_group_mappings::user_id.eq(msg.id))
        .inner_join(groups::table)
        .inner_join(domain_group_mappings::table.on(domain_group_mappings::group_id.eq(groups::id)))
        .inner_join(domains::table.on(domain_group_mappings::domain_id.eq(domains::id)))
        .select((domains::fqdn, groups::permission))
        .load::<DomainPermission>(conn)
        .map_err(|e| e.into())
});

impl_handler! (CreateGroup(conn, msg) for DbExecutor {
    let new_group = Group {
        id: CryptoUtil::generate_uuid(),
        friendly_name: msg.friendly_name,
        permission: "public".into()
    };

    diesel::insert_into(groups::table)
        .values(&new_group)
        .execute(conn)?;

    Ok(new_group)
});

impl_handler! (GetGroup(conn, msg) for DbExecutor {
    let mut group = groups::table
        .filter(groups::id.eq(&msg.id))
        .limit(1)
        .load::<Group>(conn)?;

    group.pop().ok_or(ServiceError::NotFound("group with that id not found".to_string()))
});

impl_handler! (SetGroupUsers(conn, msg) for DbExecutor {
    let group_id = msg.group_id.clone();
    
    let mappings: Vec<UserGroupMapping> = msg.user_ids.into_iter().map(|id| {
        UserGroupMapping {
            user_id: id.clone(),
            group_id: group_id.clone()
        }
    }).collect();

    diesel::delete(user_group_mappings::table)
        .filter(user_group_mappings::group_id.eq(&msg.group_id))
        .execute(conn).ok();

    diesel::insert_into(user_group_mappings::table)
        .values(&mappings)
        .execute(conn)?;

    Ok(mappings.len())
});

impl_handler! (SetGroupDomains(conn, msg) for DbExecutor {
    let group_id = msg.group_id.clone();

    let domain_ids = domains::table
        .filter(domains::fqdn.eq_any(msg.fqdns))
        .select(domains::id)
        .load::<String>(conn)?;

    let mappings: Vec<DomainGroupMapping> = domain_ids.into_iter().map(|id| {
        DomainGroupMapping {
            domain_id: id.clone(),
            group_id: group_id.clone()
        }
    }).collect();
        

    diesel::delete(domain_group_mappings::table)
        .filter(domain_group_mappings::group_id.eq(&msg.group_id))
        .execute(conn).ok();

    diesel::insert_into(domain_group_mappings::table)
        .values(&mappings)
        .execute(conn)?;

    Ok(mappings.len())
});

impl_handler! (GetUsersByGroup(conn, msg) for DbExecutor {
    user_group_mappings::table
        .filter(user_group_mappings::group_id.eq(&msg.id))
        .inner_join(users::table)
        .select((users::id, users::friendly_name, users::hashed_key))
        .load::<User>(conn)
        .map_err(|e| e.into())
});

impl_handler! (GetDomainsByGroup(conn, msg) for DbExecutor {
    domain_group_mappings::table
        .filter(domain_group_mappings::group_id.eq(&msg.id))
        .inner_join(domains::table)
        .select((domains::id, domains::fqdn, domains::hashed_fqdn))
        .load::<Domain>(conn)
        .map_err(|e| e.into())
});

impl_handler! (GetGroups(conn) for DbExecutor {
    groups::table
            .load::<Group>(conn)
            .map_err(|e| e.into())
});

impl_handler! (AddCertificateToDomain(conn, msg) for DbExecutor {
    diesel::replace_into(certificates::table)
        .values(&msg.cert)
        .execute(conn)?;

    Ok(msg.cert)
});

impl_handler! (GetCertificate(conn, msg) for DbExecutor {

    if let Some(id) = &msg.id {
        let mut certs = certificates::table
            .filter(certificates::domain_id.eq(&msg.domain_id))
            .filter(certificates::id.eq(id))
            .filter(certificates::friendly_name.eq(&msg.friendly_name))
            .limit(1)
            .load::<Certificate>(conn)?;

        return certs.pop().ok_or(ServiceError::NotFound("certificate not found".to_string()))
    } else {
        let mut certs = certificates::table
            .filter(certificates::domain_id.eq(&msg.domain_id))
            .filter(certificates::friendly_name.eq(&msg.friendly_name))
            .order(certificates::id.desc())
            .limit(1)
            .load::<Certificate>(conn)?;

        return certs.pop().ok_or(ServiceError::NotFound("certificate not found".to_string()))
    }
});

impl_handler! (GetCertificatesByDomain(conn, msg) for DbExecutor {
    Ok(certificates::table
        .filter(certificates::domain_id.eq(&msg.id))
        .load::<Certificate>(conn)?)
});

impl_handler! (GetCertificatesByDomainAndId(conn, msg) for DbExecutor {

    if let Some(id) = &msg.id {
        return Ok(certificates::table
            .filter(certificates::domain_id.eq(&msg.domain_id))
            .filter(certificates::id.eq(&id))
            .load::<Certificate>(conn)?
        )
    } else {
        // This is an extremely stupid way of getting the latest versions of certificates,
        // but I simply can't find a nice way to do nested inner joins like I would in plain sql

        let certs = certificates::table
            .filter(certificates::domain_id.eq(&msg.domain_id))
            .order(certificates::id.desc())
            .limit(10)
            .load::<Certificate>(conn)?;

        if certs.is_empty() {
            return Err(ServiceError::NotFound("no certificates found".into()))
        }

        // Get all certificates whose version is equal to the highest version found
        let latest = certs[0].id;
        return Ok(certs.into_iter().take_while(|x| x.id == latest).collect());
    }
});

impl_handler! (DeleteCertificateByPath(conn, msg) for DbExecutor {
    diesel::delete(certificates::table)
        .filter(certificates::path.eq(&msg.path))
        .execute(conn)
        .map_err(|e| e.into())
});