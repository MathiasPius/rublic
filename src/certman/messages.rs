use std::path::PathBuf;
use crate::database::models::Certificate;
use super::errors::Error;
use super::models::*;

actor_command_new! (CertificateDiscovered(path: PathBuf, fqdn: String) -> Result<Certificate, Error>);
actor_command_new! (CertificateDisappeared(path: PathBuf) -> Result<usize, Error>);
actor_command_new! (GetCertificateByPath(path: String) -> Result<SingleCertificate, Error>);