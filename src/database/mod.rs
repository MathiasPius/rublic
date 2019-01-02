pub mod models;
pub mod messages;
mod handlers;

// models.rs
use actix::{Actor, SyncContext};
use diesel::mysql::MysqlConnection;
use diesel::r2d2::{ConnectionManager, Pool};

/// This is db executor actor. can be run in parallel
pub struct DbExecutor(pub Pool<ConnectionManager<MysqlConnection>>);

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}