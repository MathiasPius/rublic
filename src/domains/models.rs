pub mod internal {
    use actix::Message;
    use serde_derive::{Serialize, Deserialize};
    use crate::errors::ServiceError;
    use crate::schema::*;
    use super::external::*;

    // Data
    #[table_name = "domain_entries"]
    #[derive(Identifiable, Queryable, Insertable, Associations)]
    pub struct DomainEntry {
        pub id: String,
        pub fqdn: String
    }

    #[derive(Identifiable, Queryable, Insertable, Associations)]
    pub struct DomainGroup {
        pub id: String,
        pub friendly_name: String
    }

    #[derive(Identifiable, Queryable, Insertable, Associations)]
    #[belongs_to(DomainEntry)]
    #[belongs_to(DomainGroup)]
    pub struct EntryGroupMapping {
        pub id: String,
        pub domain_entry_id: String,
        pub domain_group_id: String
    }


    // Commands
    #[derive(Serialize, Deserialize)]
    pub struct GetExpandedDomainEntry {
        pub id: String
    }

    impl Message for GetExpandedDomainEntry {
        type Result = Result<ExpandedDomainEntry, ServiceError>;
    }

    #[derive(Serialize, Deserialize)]
    pub struct GetAllDomainEntries { }
    impl Message for GetAllDomainEntries {
        type Result = Result<SimpleDomainEntriesList, ServiceError>;
    }


    #[derive(Serialize, Deserialize)]
    pub struct GetExpandedDomainGroup {
        pub id: String
    }

    impl Message for GetExpandedDomainGroup {
        type Result = Result<ExpandedDomainGroup, ServiceError>;
    }

    #[derive(Serialize, Deserialize)]
    pub struct GetAllDomainGroups { }
    impl Message for GetAllDomainGroups {
        type Result = Result<SimpleDomainGroupsList, ServiceError>;
    }
}


pub mod external {
    use serde_derive::{Serialize, Deserialize};

    // Commands

    // Views
    #[derive(Serialize, Deserialize)]
    pub struct SimpleDomainEntry {
        pub id: String,
        pub fqdn: String
    }

    #[derive(Serialize, Deserialize)]
    pub struct ExpandedDomainEntry {
        pub id: String,
        pub fqdn: String,
        pub groups: Vec<SimpleDomainGroup>
    }

    #[derive(Serialize, Deserialize)]
    pub struct SimpleDomainEntriesList {
        pub entries: Vec<SimpleDomainEntry>
    }


    #[derive(Serialize, Deserialize)]
    pub struct SimpleDomainGroup {
        pub id: String,
        pub friendly_name: String
    }

    #[derive(Serialize, Deserialize)]
    pub struct ExpandedDomainGroup {
        pub id: String,
        pub friendly_name: String,
        pub entries: Vec<SimpleDomainEntry>
    }

    #[derive(Serialize, Deserialize)]
    pub struct SimpleDomainGroupsList {
        pub groups: Vec<SimpleDomainGroup>
    }
}