use std::path::PathBuf;
use crate::database::models::Certificate;

actor_command! (CertificateDiscovered(path: PathBuf, fqdn: String) -> Certificate);