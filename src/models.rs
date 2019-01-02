// models.rs
use actix::{Actor, SyncContext};
use diesel::mysql::MysqlConnection;
use diesel::r2d2::{ConnectionManager, Pool};

/// This is db executor actor. can be run in parallel
pub struct DbExecutor(pub Pool<ConnectionManager<MysqlConnection>>);

// Actors communicate exclusively by exchanging messages. 
// The sending actor can optionally wait for the response. 
// Actors are not referenced directly, but by means of addresses.
// Any rust type can be an actor, it only needs to implement the Actor trait.
impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}
