use actix::Handler;
use futures::Future;
use std::io::Read;
use std::fs::File;
use openssl::x509::X509;
use chrono::{NaiveDateTime};
use crate::database::messages::{GetDomainByFqdn, DeleteCertificateByPath};
use crate::database::models::Certificate;
use crate::config::CERT_PATTERN;
use super::CertificateManager;
use super::messages::*;
use super::models::*;
use super::errors::Error;

fn parse_filename(filename: &str) -> Result<(String, i32), Error> {
    match CERT_PATTERN.captures(&filename) {
        Some(names) => {
            if names.len() != 4 {
                return Err(Error::Unknown);
            }

            if let (Some(name), Some(version), Some(ext)) = (names.get(1), names.get(2), names.get(3)) {
                if let Ok(version) = version.as_str().parse::<i32>() {
                    return Ok((format!("{}.{}", name.as_str(), ext.as_str()), version));
                } else {
                    return Err(Error::ParseError);
                }
            } else {
                return Err(Error::ParseError);
            }
        },
        None => Err(Error::Unknown)
    }
}

fn parse_date(date: &openssl::asn1::Asn1TimeRef) -> Result<NaiveDateTime, Error> {
    let datestr = &format!("{}", &date);

    match NaiveDateTime::parse_from_str(datestr, "%b %e %T %Y GMT") {
        Ok(date) => Ok(date),
        Err(_) => Err(Error::ParseError)
    }
}

fn parse_certificate(raw: &[u8]) -> Result<PemFileContents, Error> {
    match X509::from_pem(&raw) {
        Ok(cert) => {
            let not_before = parse_date(&cert.not_before());
            let not_after = parse_date(&cert.not_after());

            if let (Ok(not_before), Ok(not_after)) = (not_before, not_after) {
                return Ok(PemFileContents::PublicCertificate(
                    PublicCertificate {
                        not_before,
                        not_after
                    }
                ))
            } else {
                return Err(Error::ParseError)
            }
        },
        Err(_) => Ok(PemFileContents::PrivateKey(PrivateKey{}))
    }
}

fn read_pem_file(filename: &String) -> Result<PemFileContents, Error> {
    let mut bytes = Vec::new();

    match File::open(&filename) {
        Ok(mut file) => {
            match file.read_to_end(&mut bytes) {
                Ok(_) => parse_certificate(&bytes),
                Err(e) => Err(Error::FileError(e))
            }
        },
        Err(e) => Err(Error::FileError(e))
    }
}

impl Handler<CertificateDiscovered> for CertificateManager {
    type Result = Result<Certificate, Error>;

    fn handle(&mut self, msg: CertificateDiscovered, _: &mut Self::Context) -> Self::Result {
        let path = msg.path;
        let fqdn = msg.fqdn;

        let path_str: String = path.to_string_lossy().into();
        let filename: String = path.file_name().unwrap().to_string_lossy().into();

        match parse_filename(&filename) {
            Ok((friendly_name, version)) => {
                match read_pem_file(&path_str) {
                    Ok(contents) => {
                        return self.db.send(GetDomainByFqdn { fqdn }).flatten()
                            .and_then(|domain| {
                                match contents {
                                    PemFileContents::PublicCertificate(cert) => {
                                        return  Ok(Certificate {
                                            is_private: false,
                                            id: version,
                                            domain_id: domain.id,
                                            friendly_name,
                                            path: path_str,
                                            not_after: Some(cert.not_after),
                                            not_before: Some(cert.not_before)
                                        })
                                    },
                                    PemFileContents::PrivateKey(_) => {
                                        return Ok(Certificate {
                                            is_private: true,
                                            id: version,
                                            domain_id: domain.id,
                                            friendly_name,
                                            path: path_str,
                                            not_before: None,
                                            not_after: None
                                        })
                                    }
                                }
                            })
                            .from_err()
                            .wait()
                    },
                    Err(e) => Err(e)
                }
            },
            Err(e) => Err(e)
        }
    }
}

impl Handler<CertificateDisappeared> for CertificateManager {
    type Result = Result<(), Error>;

    fn handle(&mut self, msg: CertificateDisappeared, _: &mut Self::Context) -> Self::Result {
        self.db.send(DeleteCertificateByPath{ 
                path: msg.path.to_string_lossy().into()  
            })
            .flatten()
            .and_then(|_| Ok(()))
            .from_err()
            .wait()
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