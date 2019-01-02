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