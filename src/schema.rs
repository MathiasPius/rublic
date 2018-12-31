table! {
    access_credentials (id) {
        id -> Char,
        friendly_name -> Varchar,
        hashed_key -> Char,
    }
}

table! {
    access_groups (id) {
        id -> Char,
        friendly_name -> Varchar,
    }
}

table! {
    credential_group_mappings (id) {
        id -> Char,
        access_credential_id -> Char,
        access_group_id -> Char,
    }
}

table! {
    domains (id) {
        id -> Char,
        fqdn -> Varchar,
    }
}

table! {
    domain_groups (id) {
        id -> Char,
        friendly_name -> Varchar,
    }
}

table! {
    domain_group_mappings (domain_id, group_id) {
        domain_id -> Char,
        group_id -> Char,
    }
}

joinable!(credential_group_mappings -> access_credentials (access_credential_id));
joinable!(credential_group_mappings -> access_groups (access_group_id));
joinable!(domain_group_mappings -> domain_groups (group_id));
joinable!(domain_group_mappings -> domains (domain_id));

allow_tables_to_appear_in_same_query!(
    access_credentials,
    access_groups,
    credential_group_mappings,
    domains,
    domain_groups,
    domain_group_mappings,
);
