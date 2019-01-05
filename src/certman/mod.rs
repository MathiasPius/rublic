pub mod messages;
pub mod models;
pub mod errors;
mod handlers;

use actix::{Actor, Context, Addr};
use crate::database::DbExecutor;

pub struct CertificateManager {
    pub db: Addr<DbExecutor>
}

impl Actor for CertificateManager {
    type Context = Context<Self>;
}