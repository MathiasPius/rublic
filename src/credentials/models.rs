pub mod internal {
    use actix::Message;
    use serde_derive::{Serialize, Deserialize};
    use crate::errors::ServiceError;
    use crate::schema::{access_credentials, access_groups, credential_group_mappings};
    use super::external::*;

    // Data
    #[derive(Identifiable, Queryable, Insertable, Associations)]
    pub struct AccessCredential {
        pub id: String,
        pub friendly_name: String,
        pub hashed_key: String
    }

    #[derive(Identifiable, Queryable, Insertable, Associations)]
    pub struct AccessGroup {
        pub id: String,
        pub friendly_name: String
    }

    #[derive(Identifiable, Queryable, Insertable, Associations)]
    #[belongs_to(AccessCredential)]
    #[belongs_to(AccessGroup)]
    pub struct CredentialGroupMapping {
        pub id: String,
        pub access_credential_id: String,
        pub access_group_id: String
    }


    // Commands
    actor_command! (CreateAccessCredential(friendly_name: String, hashed_key: String) -> AccessCredential);
    actor_command! (GetAllAccessCredentials() -> SimpleAccessCredentialsList);
    actor_command! (GetExpandedAccessCredential(id: String) -> ExpandedAccessCredential);
    actor_command! (GetExpandedAccessGroup(id: String) -> ExpandedAccessGroup);
    actor_command! (GetAllAccessGroups() -> SimpleAccessGroupsList);
}


pub mod external {
    use serde_derive::{Serialize, Deserialize};

    // Commands
    #[derive(Serialize, Deserialize)]
    pub struct CreateAccessCredentialRequest {
        pub friendly_name: String
    }

    #[derive(Serialize, Deserialize)]
    pub struct NewlyCreatedAccessCredential {
        pub friendly_name: String,
        pub secret_key: String
    }

    // Views
    #[derive(Serialize, Deserialize)]
    pub struct SimpleAccessCredential {
        pub id: String,
        pub friendly_name: String
    }

    #[derive(Serialize, Deserialize)]
    pub struct ExpandedAccessCredential {
        pub id: String,
        pub friendly_name: String,
        pub groups: Vec<SimpleAccessGroup>
    }

    #[derive(Serialize, Deserialize)]
    pub struct SimpleAccessCredentialsList {
        pub credentials: Vec<SimpleAccessCredential>
    }


    #[derive(Serialize, Deserialize)]
    pub struct SimpleAccessGroup {
        pub id: String,
        pub friendly_name: String
    }

    #[derive(Serialize, Deserialize)]
    pub struct ExpandedAccessGroup {
        pub id: String,
        pub friendly_name: String,
        pub credentials: Vec<SimpleAccessCredential>
    }

    #[derive(Serialize, Deserialize)]
    pub struct SimpleAccessGroupsList {
        pub groups: Vec<SimpleAccessGroup>
    }
}