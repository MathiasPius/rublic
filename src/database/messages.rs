use super::models::*;

actor_command! (GetDomainByFqdn(fqdn: String) -> Domain);
actor_command! (GetGroupsByDomain(id: String) -> Vec<Group>);