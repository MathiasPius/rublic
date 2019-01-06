use diesel::{prelude::*};
use actix::Handler;
use crate::schema::*;
use crate::database::DbExecutor;
use crate::cryptoutil::CryptoUtil;
use super::models::*;
use super::messages::*;
use super::errors::Error;

fn exactly_one<T>(mut items: Vec<T>, name: &str) -> Result<T, Error> {
    match items.len() {
        0 => Err(Error::DataNotFound(format!("{} not found", name))),
        1 => Ok(items.pop().unwrap()),
        _ => Err(Error::TooManyresults(format!("multiple {}s found", name)))
    }
}

impl Handler<CreateDomain> for DbExecutor {
    type Result = Result<Domain, Error>;

    fn handle(&mut self, msg: CreateDomain, _: &mut Self::Context) -> Self::Result {
        self.with_connection(|conn| {
            let domain = Domain {
                id: CryptoUtil::generate_uuid(),
                hashed_fqdn: CryptoUtil::hash_string(&msg.fqdn),
                fqdn: msg.fqdn,
            };

            diesel::insert_into(domains::table)
                .values(&domain)
                .execute(conn)?;

            Ok(domain)
        })
    }
}

impl Handler<DeleteDomain> for DbExecutor {
    type Result = Result<(), Error>;

    fn handle(&mut self, msg: DeleteDomain, _: &mut Self::Context) -> Self::Result {
        self.with_connection(|conn| {
            diesel::delete(domains::table)
                .filter(domains::fqdn.eq(&msg.fqdn))
                .execute(conn)
                .map_err(|e| e.into())
                .and_then(|rows| match rows {
                    0 => Err(Error::DataNotFound("domain not found".into())),
                    _ => Ok(())
                })
        })
    }
}

impl Handler<GetDomainByFqdn> for DbExecutor {
    type Result = Result<Domain, Error>;

    fn handle(&mut self, msg: GetDomainByFqdn, _: &mut Self::Context) -> Self::Result {
        self.with_connection(|conn| {
            domains::table
                .filter(domains::fqdn.eq(&msg.fqdn))
                .load::<Domain>(conn)
                .map_err(|e| e.into())
                .and_then(move |f| exactly_one(f, "domain"))
        })
    }
}

impl Handler<GetGroupsByDomain> for DbExecutor {
    type Result = Result<Vec<Group>, Error>;

    fn handle(&mut self, msg: GetGroupsByDomain, _: &mut Self::Context) -> Self::Result {
        self.with_connection(|conn| {
            domain_group_mappings::table
                .filter(domain_group_mappings::domain_id.eq(&msg.id))
                .inner_join(groups::table)
                .select((groups::id, groups::friendly_name, groups::permission))
                .load::<Group>(conn)
                .map_err(|e| e.into())
        })
    }
}

impl Handler<GetGroupsByUser> for DbExecutor {
    type Result = Result<Vec<Group>, Error>;

    fn handle(&mut self, msg: GetGroupsByUser, _: &mut Self::Context) -> Self::Result {
        self.with_connection(|conn| {
            user_group_mappings::table
            .filter(user_group_mappings::user_id.eq(&msg.id))
            .inner_join(groups::table)
            .select((groups::id, groups::friendly_name, groups::permission))
            .load::<Group>(conn)
            .map_err(|e| e.into())    
        })
    }
}

impl Handler<CreateUser> for DbExecutor {
    type Result = Result<User, Error>;

    fn handle(&mut self, msg: CreateUser, _: &mut Self::Context) -> Self::Result {
        if msg.friendly_name == "admin" {
            return Err(Error::DataConflict("admin username is reserved".into()));
        } else {
            self.with_connection(|conn| {
                let new_user = User {
                    id: CryptoUtil::generate_uuid(),
                    friendly_name: msg.friendly_name,
                    hashed_key: msg.hashed_key
                };

                diesel::insert_into(users::table)
                    .values(&new_user)
                    .execute(conn)?;

                Ok(new_user)
            })
        }
    }
}

impl Handler<GetUserByName> for DbExecutor {
    type Result = Result<User, Error>;

    fn handle(&mut self, msg: GetUserByName, _: &mut Self::Context) -> Self::Result {
        self.with_connection(|conn| {
            users::table
                .filter(users::friendly_name.eq(&msg.friendly_name))
                .load::<User>(conn)
                .map_err(|e| e.into())
                .and_then(move |f| exactly_one(f, "user"))
        })
    }
}

impl Handler<GetUser> for DbExecutor {
    type Result = Result<User, Error>;

    fn handle(&mut self, msg: GetUser, _: &mut Self::Context) -> Self::Result {
        self.with_connection(|conn| {
            users::table
                .filter(users::id.eq(&msg.id))
                .load::<User>(conn)
                .map_err(|e| e.into())
                .and_then(move |f| exactly_one(f, "user"))
        })
    }
}


impl Handler<GetUserPermissions> for DbExecutor {
    type Result = Result<Vec<DomainPermission>, Error>;

    fn handle(&mut self, msg: GetUserPermissions, _: &mut Self::Context) -> Self::Result {
        self.with_connection(|conn| {
            user_group_mappings::table
            .filter(user_group_mappings::user_id.eq(msg.id))
            .inner_join(groups::table)
            .inner_join(domain_group_mappings::table.on(domain_group_mappings::group_id.eq(groups::id)))
            .inner_join(domains::table.on(domain_group_mappings::domain_id.eq(domains::id)))
            .select((domains::fqdn, groups::permission))
            .load::<DomainPermission>(conn)
            .map_err(|e| e.into())
        })
    }
}


impl Handler<CreateGroup> for DbExecutor {
    type Result = Result<Group, Error>;
    
    fn handle(&mut self, msg: CreateGroup, _: &mut Self::Context) -> Self::Result {
        self.with_connection(|conn| {
            let new_group = Group {
                id: CryptoUtil::generate_uuid(),
                friendly_name: msg.friendly_name,
                permission: "public".into()
            };

            diesel::insert_into(groups::table)
                .values(&new_group)
                .execute(conn)?;

            Ok(new_group)        
        })
    }
}

impl Handler<GetGroup> for DbExecutor {
    type Result = Result<Group, Error>;

    fn handle(&mut self, msg: GetGroup, _: &mut Self::Context) -> Self::Result {
        self.with_connection(|conn| {
            groups::table
                .filter(groups::id.eq(&msg.id))
                .load::<Group>(conn)
                .map_err(|e| e.into())
                .and_then(move |f| exactly_one(f, "group"))
        })
    }
}

impl Handler<SetGroupUsers> for DbExecutor {
    type Result = Result<(), Error>;

    fn handle(&mut self, msg: SetGroupUsers, _: &mut Self::Context) -> Self::Result {
        self.with_connection(|conn| {

            let mappings: Vec<UserGroupMapping> = (&msg.user_ids).into_iter().map(|id| {
                UserGroupMapping {
                    user_id: id.clone(),
                    group_id: msg.group_id.clone()
                }
            }).collect();

            diesel::delete(user_group_mappings::table)
                .filter(user_group_mappings::group_id.eq(&msg.group_id))
                .execute(conn)?;

            diesel::insert_into(user_group_mappings::table)
                .values(&mappings)
                .execute(conn)
                .map_err(|e| e.into())
                .and_then(move |rows| {
                    if rows != msg.user_ids.len() {
                        Err(Error::Unknown)
                    } else {
                        Ok(())
                    }
                })
        })
    }
}

impl Handler<SetGroupDomains> for DbExecutor {
    type Result = Result<(), Error>;

    fn handle(&mut self, msg: SetGroupDomains, _: &mut Self::Context) -> Self::Result {
        self.with_connection(|conn| {

            let mappings: Vec<DomainGroupMapping> = (&msg.fqdns).into_iter().map(|id| {
                DomainGroupMapping {
                    domain_id: id.clone(),
                    group_id: msg.group_id.clone()
                }
            }).collect();

            diesel::delete(domain_group_mappings::table)
                .filter(domain_group_mappings::group_id.eq(&msg.group_id))
                .execute(conn)?;

            diesel::insert_into(domain_group_mappings::table)
                .values(&mappings)
                .execute(conn)
                .map_err(|e| e.into())
                .and_then(move |rows| {
                    if rows != msg.fqdns.len() {
                        Err(Error::Unknown)
                    } else {
                        Ok(())
                    }
                })
        })
    }
}

impl Handler<GetUsersByGroup> for DbExecutor {
    type Result = Result<Vec<User>, Error>;

    fn handle(&mut self, msg: GetUsersByGroup, _: &mut Self:: Context) -> Self::Result {
        self.with_connection(|conn| {
            user_group_mappings::table
                .filter(user_group_mappings::group_id.eq(&msg.id))
                .inner_join(users::table)
                .select((users::id, users::friendly_name, users::hashed_key))
                .load::<User>(conn)
                .map_err(|e| e.into())
        })
    }
}

impl Handler<GetDomainsByGroup> for DbExecutor {
    type Result = Result<Vec<Domain>, Error>;

    fn handle(&mut self, msg: GetDomainsByGroup, _: &mut Self:: Context) -> Self::Result {
        self.with_connection(|conn| {
            domain_group_mappings::table
                .filter(domain_group_mappings::group_id.eq(&msg.id))
                .inner_join(domains::table)
                .select((domains::id, domains::fqdn, domains::hashed_fqdn))
                .load::<Domain>(conn)
                .map_err(|e| e.into())
        })
    }
}

impl Handler<GetGroups> for DbExecutor {
    type Result = Result<Vec<Group>, Error>;

    fn handle(&mut self, _: GetGroups, _: &mut Self::Context) -> Self::Result {
        self.with_connection(|conn| {
            groups::table
                .load::<Group>(conn)
                .map_err(|e| e.into())
        })
    }
}

impl Handler<AddCertificateToDomain> for DbExecutor {
    type Result = Result<Certificate, Error>;

    fn handle(&mut self, msg: AddCertificateToDomain, _: &mut Self::Context) -> Self::Result {
        self.with_connection(|conn| {
            diesel::replace_into(certificates::table)
                .values(&msg.cert)
                .execute(conn)?;

            Ok(msg.cert)
        })
    }
}

impl Handler<GetCertificate> for DbExecutor {
    type Result = Result<Certificate, Error>;

    fn handle(&mut self, msg: GetCertificate, _: &mut Self::Context) -> Self::Result {
        self.with_connection(|conn| {
            if let Some(id) = &msg.id {
                certificates::table
                    .filter(certificates::domain_id.eq(&msg.domain_id))
                    .filter(certificates::id.eq(id))
                    .filter(certificates::friendly_name.eq(&msg.friendly_name))
                    .limit(1)
                    .load::<Certificate>(conn)
                    .map_err(|e| e.into())
                    .and_then(move |f| exactly_one(f, "certificate"))
            } else {
                certificates::table
                    .filter(certificates::domain_id.eq(&msg.domain_id))
                    .filter(certificates::friendly_name.eq(&msg.friendly_name))
                    .order(certificates::id.desc())
                    .limit(1)
                    .load::<Certificate>(conn)
                    .map_err(|e| e.into())
                    .and_then(move |f| exactly_one(f, "certificate"))
            }
        })
    }
}

impl Handler<GetCertificatesByDomain> for DbExecutor {
    type Result = Result<Vec<Certificate>, Error>;

    fn handle(&mut self, msg: GetCertificatesByDomain, _: &mut Self::Context) -> Self::Result {
        self.with_connection(|conn| {
            certificates::table
                .filter(certificates::domain_id.eq(&msg.id))
                .load::<Certificate>(conn)
                .map_err(|e| e.into())
        })
    }        
}

impl Handler<GetCertificatesByDomainAndId> for DbExecutor {
    type Result = Result<Vec<Certificate>, Error>;

    fn handle(&mut self, msg: GetCertificatesByDomainAndId, _: &mut Self::Context) -> Self::Result {
        self.with_connection(|conn| {
            if let Some(id) = &msg.id {
                certificates::table
                    .filter(certificates::domain_id.eq(&msg.domain_id))
                    .filter(certificates::id.eq(&id))
                    .load::<Certificate>(conn)
                    .map_err(|e| e.into())
            } else {
                let latest = certificates::table
                    .filter(certificates::domain_id.eq(&msg.domain_id))
                    .order(certificates::id.desc())
                    .limit(1)
                    .select(certificates::id)
                    .load::<i32>(conn)
                    .map_err(|e| e.into())
                    .and_then(move |mut rows| match rows.len() {
                        0 => Err(Error::DataNotFound("no certificates found for domain".into())),
                        _ => Ok(rows.pop().unwrap())
                    })?;

                certificates::table
                    .filter(certificates::domain_id.eq(&msg.domain_id))
                    .filter(certificates::id.nullable().eq(latest))
                    .load::<Certificate>(conn)
                    .map_err(|e| e.into())
            }
        })
    }
}

impl Handler<DeleteCertificateByPath> for DbExecutor {
    type Result = Result<(), Error>;

    fn handle(&mut self, msg: DeleteCertificateByPath, _: &mut Self::Context) -> Self::Result {
        self.with_connection(|conn| {
            diesel::delete(certificates::table)
                .filter(certificates::path.eq(&msg.path))
                .execute(conn)
                .map_err(|e| e.into())
                .and_then(|rows| match rows {
                    0 => Err(Error::DataNotFound("domain not found".into())),
                    _ => Ok(())
                })
        })
    }
}