use regex::Regex;
use actix::Handler;
use futures::Future;
use crate::errors::ServiceError;
use crate::database::messages::{AddCertificateToDomain, GetDomainByFqdn};
use crate::database::models::Certificate;
use super::CertificateManager;
use super::messages::*;

lazy_static! {
    static ref CERT_PATTERN: Regex = Regex::new(r"(\w+)([0-9]+)\.(\w+)").unwrap();
}

impl Handler<CertificateDiscovered> for CertificateManager {
    type Result = Result<Certificate, ServiceError>;

    fn handle(&mut self, msg: CertificateDiscovered, _: &mut Self::Context) -> Self::Result {
        let path = msg.path;
        let fqdn = msg.fqdn;

        let path_str: String = path.to_string_lossy().into();
        let filename: String = path.file_name().unwrap().to_string_lossy().into();

        if let Some(names) = CERT_PATTERN.captures(&filename) {
            if names.len() != 4 {
                println!("DW: certificate name {:?} failed pattern matching", path_str);
                return Err(ServiceError::InternalServerError);
            }

            let (certname, id, fileext) = (
                names.get(1).unwrap().as_str(), 
                names.get(2).unwrap().as_str(), 
                names.get(3).unwrap().as_str()
            );

            // Lookup the fqdn and see if we the domain exists
            return self.db.send(GetDomainByFqdn{ 
                    fqdn
                }).flatten()
                // And if it does - insert the certificate
                .and_then(|domain| {
                    let certificate = Certificate {
                        id: id.parse::<i32>().unwrap(),
                        domain_id: domain.id,
                        friendly_name: format!("{}.{}", certname, fileext),
                        path: path.to_string_lossy().into(),
                        not_before: chrono::NaiveDate::from_ymd(2018, 6, 2).and_hms(13, 37, 0),
                        not_after: chrono::NaiveDate::from_ymd(2019, 6, 2).and_hms(13, 37, 0)
                    };

                    self.db
                        .send(AddCertificateToDomain { cert: certificate })
                        .map_err(|e| e.into())
                })
                .flatten().wait();
        }

        Err(ServiceError::InternalServerError)
    }
}