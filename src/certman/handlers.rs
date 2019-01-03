use regex::Regex;
use actix::Handler;
use futures::Future;
use std::io::Read;
use std::fs::File;
use openssl::x509::X509;
use chrono::{NaiveDate, NaiveDateTime};
use crate::errors::ServiceError;
use crate::database::messages::{AddCertificateToDomain, GetDomainByFqdn, DeleteCertificateByPath};
use crate::database::models::Certificate;
use super::CertificateManager;
use super::messages::*;
use super::models::*;

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


        // This feels like a really dirty way to parse dates
        let mut not_before: NaiveDateTime = NaiveDate::from_ymd(1970, 1, 1).and_hms(0, 0, 0);
        let mut not_after: NaiveDateTime = NaiveDate::from_ymd(1970, 1, 1).and_hms(0, 0, 0);

        let mut bytes = Vec::new();
        if let Ok(mut file) = File::open(&path) {
            if let Ok(_) = file.read_to_end(&mut bytes) {
                if let Ok(parsed_cert) = X509::from_pem(&bytes[..]) {
                    let unparsed_before = &format!("{}", &parsed_cert.not_before());
                    let unparsed_after = &format!("{}", &parsed_cert.not_after());
                    not_before = NaiveDateTime::parse_from_str(unparsed_before, "%b %e %T %Y GMT").unwrap();
                    not_after = NaiveDateTime::parse_from_str(unparsed_after, "%b %e %T %Y GMT").unwrap();
                }
            }
        };

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
                        not_before,
                        not_after
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

impl Handler<CertificateDisappeared> for CertificateManager {
    type Result = Result<usize, ServiceError>;

    fn handle(&mut self, msg: CertificateDisappeared, _: &mut Self::Context) -> Self::Result {
        self.db.send(DeleteCertificateByPath{ 
                path: msg.path.to_string_lossy().into()  
            }).flatten().wait()
    }
}

impl Handler<GetCertificateByPath> for CertificateManager {
    type Result = Result<SingleCertificate, ServiceError>;

    fn handle(&mut self, msg: GetCertificateByPath, _: &mut Self::Context) -> Self::Result {
        let mut bytes = Vec::new();
        if let Ok(mut file) = File::open(&msg.path) {
            if let Ok(_) = file.read_to_end(&mut bytes) {
                // Verify that the file is actually a real certificate
                if let Ok(_) = X509::from_pem(&bytes[..]) {
                    return Ok(SingleCertificate {
                        raw_data: bytes
                    });
                }
            }
        };

        Err(ServiceError::InternalServerError)
    }
}