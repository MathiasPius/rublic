use super::models::*;


// This macro just expands some function declaration-like syntax into an
// actor message implementation to avoid repetitive boilerplate code
macro_rules! actor_command {
    ($command:ident( $($names:ident : $types:ty),* ) -> $output:ty) => {
        pub struct $command {
            $(pub $names : $types),*
        }

        impl actix::Message for $command {
            type Result = Result<$output, crate::errors::ServiceError>;
        }
    }
}

actor_command! (CreateDomain(fqdn: String) -> Domain);
actor_command! (GetDomainByFqdn(fqdn: String) -> Domain);

actor_command! (CreateUser(friendly_name: String, hashed_key: String) -> User);
actor_command! (GetUserByName(friendly_name: String) -> User);
actor_command! (GetUser(id: String) -> User);

actor_command! (CreateGroup(friendly_name: String) -> Group);
actor_command! (GetGroup(id: String) -> Group);
actor_command! (GetGroupsByDomain(id: String) -> Vec<Group>);
actor_command! (GetGroupsByUser(id: String) -> Vec<Group>);
actor_command! (AddUsersToGroup(user_ids: Vec<String>, group_id: String) -> usize);
actor_command! (AddDomainsToGroup(domain_ids: Vec<String>, group_id: String) -> usize);
actor_command! (GetUsersByGroup(id: String) -> Vec<User>);
actor_command! (GetDomainsByGroup(id: String) -> Vec<Domain>);
actor_command! (GetGroups() -> Vec<Group>);