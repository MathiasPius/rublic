use actix::Handler;
use diesel::{MysqlConnection, prelude::*};
use uuid::Uuid;
use std::iter::Iterator;
//use itertools::Itertools;
//use std::collections::HashMap;
use crate::models::DbExecutor;
use crate::errors::ServiceError;
use crate::credentials::models::{internal::*, external::*};

impl Handler<CreateAccessCredential> for DbExecutor {
    type Result = Result<AccessCredential, ServiceError>;

    fn handle(&mut self, msg: CreateAccessCredential, _: &mut Self::Context) -> Self::Result {
        use crate::schema::access_credentials::dsl::*;

        let conn: &MysqlConnection = &self.0.get().unwrap();

        let new_credential = AccessCredential {
            id: Uuid::new_v4().to_string(),
            friendly_name: msg.friendly_name.clone(),
            hashed_key: msg.hashed_key
        };

        diesel::insert_into(access_credentials)
            .values(&new_credential)
            .execute(conn)
            .map_err(|error| {
                println!("{:#?}",error);
                ServiceError::InternalServerError
            })?;

        let mut items = access_credentials
            .filter(id.eq(&new_credential.id))
            .load::<AccessCredential>(conn)
            .map_err(|_| ServiceError::InternalServerError)?;

        Ok(items.pop().unwrap())
    }
}

impl Handler<GetAllAccessCredentials> for DbExecutor {
    type Result = Result<SimpleAccessCredentialsList, ServiceError>;

    fn handle(&mut self, _: GetAllAccessCredentials, _: &mut Self::Context) -> Self::Result {
        use crate::schema::*;

        let conn: &MysqlConnection = &self.0.get().unwrap();

        let items = access_credentials::table
            .load::<AccessCredential>(conn)
            .map_err(|_| ServiceError::InternalServerError)?
            .into_iter()
            .map(|c| SimpleAccessCredential {
                id: c.id,
                friendly_name: c.friendly_name
            })
            .collect();

        Ok(SimpleAccessCredentialsList {
            credentials: items
        })
    }
}

impl Handler<GetExpandedAccessCredential> for DbExecutor {
    type Result = Result<ExpandedAccessCredential, ServiceError>;

    fn handle(&mut self, msg: GetExpandedAccessCredential, _: &mut Self::Context) -> Self::Result {
        use crate::schema::*;

        let conn: &MysqlConnection = &self.0.get().unwrap();

        let credential = access_credentials::table
            .filter(access_credentials::id.eq(&msg.id))
            .limit(1)
            .load::<AccessCredential>(conn)
            .map_err(|_| ServiceError::InternalServerError)?
            .pop().ok_or(ServiceError::InternalServerError)?;

        let groups = credential_group_mappings::table
            .filter(credential_group_mappings::access_credential_id.eq(&msg.id))
            .inner_join(access_groups::table)
            .select((access_groups::id, access_groups::friendly_name))
            .load::<AccessGroup>(conn)
            .map_err(|_| ServiceError::InternalServerError)?; 

        Ok(ExpandedAccessCredential {
            id: credential.id,
            friendly_name: credential.friendly_name,
            groups: groups.into_iter().map(|group| SimpleAccessGroup {
                id: group.id,
                friendly_name: group.friendly_name
            }).collect()
        })
    }
}



impl Handler<GetExpandedAccessGroup> for DbExecutor {
    type Result = Result<ExpandedAccessGroup, ServiceError>;

    fn handle(&mut self, msg: GetExpandedAccessGroup, _: &mut Self::Context) -> Self::Result {
        use crate::schema::*;

        let conn: &MysqlConnection = &self.0.get().unwrap();

        let group = access_groups::table
            .filter(access_groups::id.eq(&msg.id))
            .limit(1)
            .load::<AccessGroup>(conn)
            .map_err(|_| ServiceError::InternalServerError)?
            .pop().ok_or(ServiceError::InternalServerError)?;

        let credentials = credential_group_mappings::table
            .filter(credential_group_mappings::access_group_id.eq(&msg.id))
            .inner_join(access_credentials::table)
            .select((access_credentials::id, access_credentials::friendly_name, access_credentials::hashed_key))
            .load::<AccessCredential>(conn)
            .map_err(|_| ServiceError::InternalServerError)?; 

        Ok(ExpandedAccessGroup {
            id: group.id,
            friendly_name: group.friendly_name,
            credentials: credentials.into_iter().map(|credential| SimpleAccessCredential {
                id: credential.id,
                friendly_name: credential.friendly_name
            }).collect()
        })
    }
}

impl Handler<GetAllAccessGroups> for DbExecutor {
    type Result = Result<SimpleAccessGroupsList, ServiceError>;

    fn handle(&mut self, _: GetAllAccessGroups, _: &mut Self::Context) -> Self::Result {
        use crate::schema::*;

        let conn: &MysqlConnection = &self.0.get().unwrap();

        let items = access_groups::table
            .load::<AccessGroup>(conn)
            .map_err(|_| ServiceError::InternalServerError)?
            .into_iter()
            .map(|c| SimpleAccessGroup {
                id: c.id,
                friendly_name: c.friendly_name
            })
            .collect();

        Ok(SimpleAccessGroupsList {
            groups: items
        })
    }
}






/*
        let credentials = access_credentials::table
            .load::<AccessCredential>(conn)
            .map_err(|_| ServiceError::InternalServerError)?
            .map(move |credential| (credential.id, credential))
            .into_iter()
            .into_group_map();

            let groups = credential_group_mappings::table
            .filter(&credential)
            .inner_join(access_groups::table)
            .select((credential_group_mappings::access_credential_id, (access_groups::id, access_groups::friendly_name)))
            .load::<(String, (String, String))>(conn)
            .map_err(|_| ServiceError::InternalServerError)?
            .into_iter()
            .into_group_map(); 
        */