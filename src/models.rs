// models.rs
use actix::{Actor, SyncContext};
use diesel::mysql::MysqlConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use serde_derive::Deserialize;

/// This is db executor actor. can be run in parallel
pub struct DbExecutor(pub Pool<ConnectionManager<MysqlConnection>>);

// Actors communicate exclusively by exchanging messages. 
// The sending actor can optionally wait for the response. 
// Actors are not referenced directly, but by means of addresses.
// Any rust type can be an actor, it only needs to implement the Actor trait.
impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

#[derive(Deserialize)]
pub struct ExpansionOption {
    pub expand: String
}


// This macro just expands some function declaration-like syntax into an
// actor message implementation to avoid repetitive boilerplate code
#[macro_export]
macro_rules! actor_command {
    ($command:ident( $($names:ident : $types:ty),* ) -> $output:ty) => {
        #[derive(Serialize, Deserialize)]
        pub struct $command {
            $(pub $names : $types),*
        }

        impl Message for $command {
            type Result = Result<$output, ServiceError>;
        }
    }
}