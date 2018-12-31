pub mod internal {
    use actix::Message;
    use serde_derive::{Serialize, Deserialize};
    use crate::errors::ServiceError;
    use crate::schema::{access_credentials, access_groups, credential_group_mappings};
    use super::external::*;

    // Datastructures
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
    #[derive(Serialize, Deserialize)]
    pub struct CreateAccessCredential {
        pub friendly_name: String,
        pub hashed_key: String
    }

    impl Message for CreateAccessCredential {
        type Result = Result<AccessCredential, ServiceError>;
    }

    #[derive(Serialize, Deserialize)]
    pub struct GetAllAccessCredentials { }
    impl Message for GetAllAccessCredentials {
        type Result = Result<SimpleAccessCredentialsList, ServiceError>;
    }

    #[derive(Serialize, Deserialize)]
    pub struct GetExpandedAccessCredential {
        pub id: String
    }

    impl Message for GetExpandedAccessCredential {
        type Result = Result<ExpandedAccessCredential, ServiceError>;
    }

    #[derive(Serialize, Deserialize)]
    pub struct GetExpandedAccessGroup {
        pub id: String
    }

    impl Message for GetExpandedAccessGroup {
        type Result = Result<ExpandedAccessGroup, ServiceError>;
    }

    #[derive(Serialize, Deserialize)]
    pub struct GetAllAccessGroups { }
    impl Message for GetAllAccessGroups {
        type Result = Result<SimpleAccessGroupsList, ServiceError>;
    }
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