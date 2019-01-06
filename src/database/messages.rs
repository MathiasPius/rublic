use super::models::*;
use super::errors::Error;

actor_command_new! (CreateDomain(fqdn: String) -> Result<Domain, Error>);
actor_command_new! (DeleteDomain(fqdn: String) -> Result<(), Error>);
actor_command_new! (GetDomainByFqdn(fqdn: String) -> Result<Domain, Error>);

actor_command_new! (CreateUser(friendly_name: String, hashed_key: String) -> Result<User, Error>);
actor_command_new! (GetUserByName(friendly_name: String) -> Result<User, Error>);
actor_command_new! (GetUser(id: String) -> Result<User, Error>);
actor_command_new! (GetUserPermissions(id: String) -> Result<Vec<DomainPermission>, Error>);

actor_command_new! (CreateGroup(friendly_name: String) -> Result<Group, Error>);
actor_command_new! (GetGroup(id: String) -> Result<Group, Error>);
actor_command_new! (GetGroupsByDomain(id: String) -> Result<Vec<Group>, Error>);
actor_command_new! (GetGroupsByUser(id: String) -> Result<Vec<Group>, Error>);
actor_command_new! (SetGroupUsers(user_ids: Vec<String>, group_id: String) -> Result<(), Error>);
actor_command_new! (SetGroupDomains(fqdns: Vec<String>, group_id: String) -> Result<(), Error>);
actor_command_new! (GetUsersByGroup(id: String) -> Result<Vec<User>, Error>);
actor_command_new! (GetDomainsByGroup(id: String) -> Result<Vec<Domain>, Error>);
actor_command_new! (GetGroups() -> Result<Vec<Group>, Error>);

actor_command_new! (AddCertificateToDomain(cert: Certificate) -> Result<Certificate, Error>);
actor_command_new! (DeleteCertificateByPath(path: String) -> Result<(), Error>);
actor_command_new! (GetCertificatesByDomain(id: String) -> Result<Vec<Certificate>, Error>);
actor_command_new! (GetCertificatesByDomainAndId(domain_id: String, id: Option<i32>) -> Result<Vec<Certificate>, Error>);
actor_command_new! (GetCertificate(domain_id: String, id: Option<i32>, friendly_name: String) -> Result<Certificate, Error>);