use actix::{Handler, Message};
use diesel::{MysqlConnection, prelude::*};
use uuid::Uuid;
use crate::models::DbExecutor;
use crate::errors::ServiceError;
use crate::certificates::models::{Certificate, NewCertificateEntry, GetCertificateById};

impl Message for NewCertificateEntry {
    type Result = Result<Certificate, ServiceError>;
}

impl Handler<NewCertificateEntry> for DbExecutor {
    type Result = Result<Certificate, ServiceError>;

    fn handle(&mut self, msg: NewCertificateEntry, _: &mut Self::Context) -> Self::Result {
        use crate::schema::certificates::dsl::*;

        let conn: &MysqlConnection = &self.0.get().unwrap();

        let new_certificate = Certificate {
            id: Uuid::new_v4().to_string(),
            filepath: msg.filepath.clone()
        };

        diesel::insert_into(certificates)
            .values(&new_certificate)
            .execute(conn)
            .map_err(|error| {
                println!("{:#?}",error); // for debugging purposes
                ServiceError::InternalServerError
            })?;

        let mut items = certificates
            .filter(id.eq(&new_certificate.id))
            .load::<Certificate>(conn)
            .map_err(|_| ServiceError::InternalServerError)?;

        Ok(items.pop().unwrap())
    }
}


impl Message for GetCertificateById {
    type Result = Result<Certificate, ServiceError>;
}

impl Handler<GetCertificateById> for DbExecutor {
    type Result = Result<Certificate, ServiceError>;

    fn handle(&mut self, msg: GetCertificateById, _: &mut Self::Context) -> Self::Result {
        use crate::schema::certificates::dsl::*;

        let conn: &MysqlConnection = &self.0.get().unwrap();

        certificates
            .filter(id.eq(&msg.id))
            .load::<Certificate>(conn)
            .map_err(|_| ServiceError::InternalServerError)
            .and_then(|mut result| {
                if let Some(certificate) = result.pop() {
                    return Ok(certificate);
                }

                Err(ServiceError::NotFound(format!("certificate with id {:?} not found", msg.id)))
            })
    }
}