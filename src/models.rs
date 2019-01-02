// This macro just expands some function declaration-like syntax into an
// actor message implementation to avoid repetitive boilerplate code
#[macro_export] macro_rules! actor_command {
    ($command:ident( $($names:ident : $types:ty),* ) -> $output:ty) => {
        pub struct $command {
            $(pub $names : $types),*
        }

        impl actix::Message for $command {
            type Result = Result<$output, crate::errors::ServiceError>;
        }
    }
}

// Simple rule for wrapping Handler implementations
#[macro_export] macro_rules! impl_handler {   
    ($message:ident ( $conn:ident, $msg:ident, $ctx:ident ) for $actor:ty $blk:block) => {
        impl actix::Handler<$message> for $actor {
            type Result = <$message as actix::Message>::Result;

            fn handle(&mut self, msg: $message, $ctx: &mut Self::Context) -> Self::Result {
                use crate::schema::*;
                let conn: &diesel::mysql::MysqlConnection = &self.0.get().unwrap();

                let inner = |$msg: $message, $conn: &diesel::mysql::MysqlConnection| $blk;

                inner(msg, conn)
            }
        }
    };
    ($message:ident ( $conn:ident, $msg:ident ) for $actor:ty $blk:block) => {
        impl_handler! ($message( $conn, $msg, _ctx) for $actor $blk);
    };
    ($message:ident ( $conn:ident ) for $actor:ty $blk:block) => {
        impl_handler! ($message( $conn, _msg, _ctx) for $actor $blk);
    }
}