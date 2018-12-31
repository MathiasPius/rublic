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

    /*
    #[derive(Identifiable, Queryable, Insertable, Associations)]
    #[belongs_to(DomainGroup)]
    #[belongs_to(AccessGroup)]
    pub struct GroupPermission {
        pub id: String,
        pub access_group_id: String,
        pub domain_group_id: String
    }
    */

    // Commands
    actor_command! (GetExpandedDomainEntry(id: String) -> ExpandedDomainEntry);
    actor_command! (GetAllDomainEntries() -> SimpleDomainEntriesList);
    actor_command! (GetExpandedDomainGroup(id: String) -> ExpandedDomainGroup);
    actor_command! (GetAllDomainGroups() -> SimpleDomainGroupsList);
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