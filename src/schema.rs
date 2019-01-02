table! {
    domains (id) {
        id -> Char,
        fqdn -> Varchar,
        hashed_fqdn -> Char,
    }
}

table! {
    domain_group_mappings (domain_id, group_id) {
        domain_id -> Char,
        group_id -> Char,
    }
}

table! {
    groups (id) {
        id -> Char,
        friendly_name -> Varchar,
        permission -> Varchar,
    }
}

table! {
    users (id) {
        id -> Char,
        friendly_name -> Varchar,
        hashed_key -> Char,
    }
}

table! {
    user_group_mappings (user_id, group_id) {
        user_id -> Char,
        group_id -> Char,
    }
}

joinable!(domain_group_mappings -> domains (domain_id));
joinable!(domain_group_mappings -> groups (group_id));
joinable!(user_group_mappings -> groups (group_id));
joinable!(user_group_mappings -> users (user_id));

allow_tables_to_appear_in_same_query!(
    domains,
    domain_group_mappings,
    groups,
    users,
    user_group_mappings,
);
