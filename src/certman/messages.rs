use std::path::PathBuf;
use crate::database::models::Certificate;
use super::models::*;

actor_command! (CertificateDiscovered(path: PathBuf, fqdn: String) -> Certificate);
actor_command! (CertificateDisappeared(path: PathBuf) -> usize);
actor_command! (GetCertificateByPath(path: String) -> SingleCertificate);