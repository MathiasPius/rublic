use super::models::*;

actor_command! (CreateDomain(fqdn: String) -> Domain);
actor_command! (GetDomainByFqdn(fqdn: String) -> Domain);

actor_command! (CreateUser(friendly_name: String, hashed_key: String) -> User);
actor_command! (GetUserByName(friendly_name: String) -> User);
actor_command! (GetUser(id: String) -> User);

actor_command! (CreateGroup(friendly_name: String) -> Group);
actor_command! (GetGroup(id: String) -> Group);
actor_command! (GetGroupsByDomain(id: String) -> Vec<Group>);
actor_command! (GetGroupsByUser(id: String) -> Vec<Group>);