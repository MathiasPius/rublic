use actix::Handler;
use futures::{IntoFuture, Future};
use std::io::Read;
use std::fs::File;
use openssl::x509::X509;
use chrono::{NaiveDateTime};
use crate::database::messages::{AddCertificateToDomain, GetDomainByFqdn, DeleteCertificateByPath};
use crate::database::models::Certificate;
use crate::config::CERT_PATTERN;
use super::CertificateManager;
use super::messages::*;
use super::models::*;
use super::errors::Error;

impl Handler<CertificateDiscovered> for CertificateManager {
    type Result = Result<Certificate, Error>;

    fn handle(&mut self, msg: CertificateDiscovered, _: &mut Self::Context) -> Self::Result {
        let path = msg.path;
        let fqdn = msg.fqdn;

        let path_str: String = path.to_string_lossy().into();
        let filename: String = path.file_name().unwrap().to_string_lossy().into();

        // This feels like a really dirty way to parse dates
        let mut not_before = None;
        let mut not_after = None;
        let mut is_private = true;

        let mut bytes = Vec::new();
        if let Ok(mut file) = File::open(&path) {
            if file.read_to_end(&mut bytes).is_ok() {
                if let Ok(parsed_cert) = X509::from_pem(&bytes[..]) {
                    // If we could parse it as an X509 Certificate, it was not a private key.
                    is_private = false;

                    let unparsed_before = &format!("{}", &parsed_cert.not_before());
                    let unparsed_after = &format!("{}", &parsed_cert.not_after());
                    not_before = Some(NaiveDateTime::parse_from_str(unparsed_before, "%b %e %T %Y GMT").unwrap());
                    not_after = Some(NaiveDateTime::parse_from_str(unparsed_after, "%b %e %T %Y GMT").unwrap());
                }
            }
        };

        if let Some(names) = CERT_PATTERN.captures(&filename) {
            if names.len() != 4 {
                println!("DW: certificate name {:?} failed pattern matching", &path_str);
                return Err(Error::Unknown);
            }

            let (certname, id, fileext) = (
                names.get(1).unwrap().as_str(), 
                names.get(2).unwrap().as_str(), 
                names.get(3).unwrap().as_str()
            );
            /*
            // Lookup the fqdn and see if we the domain exists
            return self.db.send(GetDomainByFqdn{ 
                    fqdn
                }).flatten()
                .map_err(|e| Err(Error::Unknown))
                // And if it does - insert the certificate
                .and_then(|domain| {
                    let certificate = Certificate {
                        id: id.parse::<i32>().unwrap(),
                        domain_id: domain.id,
                        friendly_name: format!("{}.{}", certname, fileext),
                        path: path.to_string_lossy().into(),
                        is_private,
                        not_before,
                        not_after
                    };

                    self.db
                        .send(AddCertificateToDomain { cert: certificate }).flatten()
                        .map_err(|e| Err(Error::Unknown))
                })
                .map_err(|_| Err(Error::Unknown));
            */
            return Err(Error::Unknown);
        }

        Err(Error::Unknown)
    }
}

impl Handler<CertificateDisappeared> for CertificateManager {
    type Result = Result<usize, Error>;

    fn handle(&mut self, msg: CertificateDisappeared, _: &mut Self::Context) -> Self::Result {
        self.db.send(DeleteCertificateByPath{ 
                path: msg.path.to_string_lossy().into()  
            }).flatten().wait();

        Ok(0)
    }
}

impl Handler<GetCertificateByPath> for CertificateManager {
    type Result = Result<SingleCertificate, Error>;

    fn handle(&mut self, msg: GetCertificateByPath, _: &mut Self::Context) -> Self::Result {
        let mut bytes = Vec::new();
        
        match File::open(&msg.path) {
            Ok(mut file) => {
                match file.read_to_end(&mut bytes) {
                    Ok(_) => Ok(SingleCertificate {
                        raw_data: bytes
                    }),
                    Err(e) => Err(Error::FileError(e))
                }
            },
            Err(e) => Err(Error::FileError(e))
        }
    }
}