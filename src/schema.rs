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
    domain_entries (id) {
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
    entry_group_mappings (id) {
        id -> Char,
        domain_entry_id -> Char,
        domain_group_id -> Char,
    }
}

table! {
    group_permissions (id) {
        id -> Char,
        permission -> Varchar,
        access_group_id -> Char,
        domain_group_id -> Char,
    }
}

joinable!(credential_group_mappings -> access_credentials (access_credential_id));
joinable!(credential_group_mappings -> access_groups (access_group_id));
joinable!(entry_group_mappings -> domain_entries (domain_entry_id));
joinable!(entry_group_mappings -> domain_groups (domain_group_id));
joinable!(group_permissions -> access_groups (access_group_id));
joinable!(group_permissions -> domain_groups (domain_group_id));

allow_tables_to_appear_in_same_query!(
    access_credentials,
    access_groups,
    credential_group_mappings,
    domain_entries,
    domain_groups,
    entry_group_mappings,
    group_permissions,
);
