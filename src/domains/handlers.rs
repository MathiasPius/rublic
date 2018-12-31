use actix::Handler;
use diesel::{MysqlConnection, prelude::*};
//use uuid::Uuid;
use std::iter::Iterator;
//use itertools::Itertools;
//use std::collections::HashMap;
use crate::models::DbExecutor;
use crate::errors::ServiceError;
use crate::domains::models::{internal::*, external::*};

impl Handler<GetExpandedDomainEntry> for DbExecutor {
    type Result = Result<ExpandedDomainEntry, ServiceError>;

    fn handle(&mut self, msg: GetExpandedDomainEntry, _: &mut Self::Context) -> Self::Result {
        use crate::schema::*;

        let conn: &MysqlConnection = &self.0.get().unwrap();

        let entry = domain_entries::table
            .filter(domain_entries::id.eq(&msg.id))
            .limit(1)
            .load::<DomainEntry>(conn)
            .map_err(|_| ServiceError::InternalServerError)?
            .pop().ok_or(ServiceError::NotFound("domain not found".into()))?;

        let groups = entry_group_mappings::table
            .filter(entry_group_mappings::domain_entry_id.eq(&msg.id))
            .inner_join(domain_groups::table)
            .select((domain_groups::id, domain_groups::friendly_name))
            .load::<DomainGroup>(conn)
            .map_err(|_| ServiceError::InternalServerError)?; 

        Ok(ExpandedDomainEntry {
            id: entry.id,
            fqdn: entry.fqdn,
            groups: groups.into_iter().map(|group| SimpleDomainGroup {
                id: group.id,
                friendly_name: group.friendly_name
            }).collect()
        })
    }
}

impl Handler<GetAllDomainEntries> for DbExecutor {
    type Result = Result<SimpleDomainEntriesList, ServiceError>;

    fn handle(&mut self, _: GetAllDomainEntries, _: &mut Self::Context) -> Self::Result {
        use crate::schema::*;

        let conn: &MysqlConnection = &self.0.get().unwrap();

        let domains = domain_entries::table
            .load::<DomainEntry>(conn)
            .map_err(|_| ServiceError::InternalServerError)?;

        Ok(SimpleDomainEntriesList {
            entries: domains.into_iter().map(|domain| SimpleDomainEntry {
                id: domain.id,
                fqdn: domain.fqdn
            }).collect()
        })
    }
}

impl Handler<GetExpandedDomainGroup> for DbExecutor {
    type Result = Result<ExpandedDomainGroup, ServiceError>;

    fn handle(&mut self, msg: GetExpandedDomainGroup, _: &mut Self::Context) -> Self::Result {
        use crate::schema::*;

        let conn: &MysqlConnection = &self.0.get().unwrap();

        let group = domain_groups::table
            .filter(domain_groups::id.eq(&msg.id))
            .limit(1)
            .load::<DomainGroup>(conn)
            .map_err(|_| ServiceError::InternalServerError)?
            .pop().ok_or(ServiceError::NotFound("group not found".into()))?;

        let domains = entry_group_mappings::table
            .filter(entry_group_mappings::domain_group_id.eq(&msg.id))
            .inner_join(domain_entries::table)
            .select((domain_entries::id, domain_entries::fqdn))
            .load::<DomainEntry>(conn)
            .map_err(|_| ServiceError::InternalServerError)?; 

        Ok(ExpandedDomainGroup {
            id: group.id,
            friendly_name: group.friendly_name,
            entries: domains.into_iter().map(|domain| SimpleDomainEntry {
                id: domain.id,
                fqdn: domain.fqdn
            }).collect()
        })
    }
}

impl Handler<GetAllDomainGroups> for DbExecutor {
    type Result = Result<SimpleDomainGroupsList, ServiceError>;

    fn handle(&mut self, _: GetAllDomainGroups, _: &mut Self::Context) -> Self::Result {
        use crate::schema::*;

        let conn: &MysqlConnection = &self.0.get().unwrap();

        let groups = domain_groups::table
            .load::<DomainGroup>(conn)
            .map_err(|_| ServiceError::InternalServerError)?;

        Ok(SimpleDomainGroupsList {
            groups: groups.into_iter().map(|domain| SimpleDomainGroup {
                id: domain.id,
                friendly_name: domain.friendly_name
            }).collect()
        })
    }
}