use super::models::*;

actor_command! (CreateDomain(fqdn: String) -> Domain);
actor_command! (DeleteDomain(fqdn: String) -> usize);
actor_command! (GetDomainByFqdn(fqdn: String) -> Domain);

actor_command! (CreateUser(friendly_name: String, hashed_key: String) -> User);
actor_command! (GetUserByName(friendly_name: String) -> User);
actor_command! (GetUser(id: String) -> User);

actor_command! (CreateGroup(friendly_name: String) -> Group);
actor_command! (GetGroup(id: String) -> Group);
actor_command! (GetGroupsByDomain(id: String) -> Vec<Group>);
actor_command! (GetGroupsByUser(id: String) -> Vec<Group>);
actor_command! (SetGroupUsers(user_ids: Vec<String>, group_id: String) -> usize);
actor_command! (SetGroupDomains(domain_ids: Vec<String>, group_id: String) -> usize);
actor_command! (GetUsersByGroup(id: String) -> Vec<User>);
actor_command! (GetDomainsByGroup(id: String) -> Vec<Domain>);
actor_command! (GetGroups() -> Vec<Group>);

actor_command! (AddCertificateToDomain(cert: Certificate) -> Certificate);
actor_command! (GetCertificatesByDomain(domain_id: String) -> Vec<Certificate>);