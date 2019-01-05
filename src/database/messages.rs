use super::models::*;
use super::errors::Error;

actor_command_new! (CreateDomain(fqdn: String) -> Result<Domain, Error>);
actor_command_new! (DeleteDomain(fqdn: String) -> Result<(), Error>);
actor_command_new! (GetDomainByFqdn(fqdn: String) -> Result<Domain, Error>);

actor_command! (CreateUser(friendly_name: String, hashed_key: String) -> User);
actor_command! (GetUserByName(friendly_name: String) -> User);
actor_command! (GetUser(id: String) -> User);
actor_command! (GetUserPermissions(id: String) -> Vec<DomainPermission>);

actor_command! (CreateGroup(friendly_name: String) -> Group);
actor_command! (GetGroup(id: String) -> Group);
actor_command! (GetGroupsByDomain(id: String) -> Vec<Group>);
actor_command! (GetGroupsByUser(id: String) -> Vec<Group>);
actor_command! (SetGroupUsers(user_ids: Vec<String>, group_id: String) -> usize);
actor_command! (SetGroupDomains(fqdns: Vec<String>, group_id: String) -> usize);
actor_command! (GetUsersByGroup(id: String) -> Vec<User>);
actor_command! (GetDomainsByGroup(id: String) -> Vec<Domain>);
actor_command! (GetGroups() -> Vec<Group>);

actor_command! (AddCertificateToDomain(cert: Certificate) -> Certificate);
actor_command! (DeleteCertificateByPath(path: String) -> usize);
actor_command! (GetCertificatesByDomain(id: String) -> Vec<Certificate>);
actor_command! (GetCertificatesByDomainAndId(domain_id: String, id: Option<i32>) -> Vec<Certificate>);
actor_command! (GetCertificate(domain_id: String, id: Option<i32>, friendly_name: String) -> Certificate);