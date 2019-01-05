pub mod models;
pub mod messages;
pub mod errors;
mod handlers;

// models.rs
use actix::{Actor, SyncContext};
use diesel::mysql::MysqlConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use self::errors::Error;

/// This is db executor actor. can be run in parallel
pub struct DbExecutor(pub Pool<ConnectionManager<MysqlConnection>>);

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

impl DbExecutor {
    fn with_connection<T, F>(&mut self, func: F) -> Result<T, Error>
        where F: FnOnce(&MysqlConnection) -> Result<T, Error> {
        match self.0.get() {
            Ok(conn) => match func(&conn) {
                Ok(result) => Ok(result),
                Err(e) => Err(e)
            },
            Err(_) => Err(Error::Unknown)
        }
    }
}