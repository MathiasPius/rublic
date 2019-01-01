// models.rs
use actix::{Actor, SyncContext};
use actix_web::{FutureResponse, HttpResponse, AsyncResponder};
use diesel::mysql::MysqlConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use futures::future::Future;
use crate::errors::ServiceError;

/// This is db executor actor. can be run in parallel
pub struct DbExecutor(pub Pool<ConnectionManager<MysqlConnection>>);

// Actors communicate exclusively by exchanging messages. 
// The sending actor can optionally wait for the response. 
// Actors are not referenced directly, but by means of addresses.
// Any rust type can be an actor, it only needs to implement the Actor trait.
impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
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

pub fn into_api_response<T: serde::Serialize>(response: impl Future<Item = T, Error = ServiceError> + 'static) 
    -> FutureResponse<HttpResponse> {
    response
        .and_then(|result|
            Ok(HttpResponse::Ok().json(result))
        )
        .from_err()
        .responder()
}