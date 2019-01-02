use crate::schema::*;

#[derive(Identifiable, Queryable, Insertable, Associations)]
pub struct User {
    pub id: String,
    pub friendly_name: String,
    pub hashed_key: String
}

#[derive(Identifiable, Queryable, Insertable, Associations)]
pub struct Domain {
    pub id: String,
    pub fqdn: String,
    pub hashed_fqdn: String
}

#[derive(Identifiable, Queryable, Insertable, Associations)]
pub struct Group {
    pub id: String,
    pub friendly_name: String,
    pub permission: String,
}

#[derive(Identifiable, Queryable, Insertable, Associations)]
#[primary_key(domain_id, group_id)]
#[belongs_to(Domain)]
#[belongs_to(Group)]
pub struct DomainGroupMapping {
    pub domain_id: String,
    pub group_id: String
}

#[derive(Identifiable, Queryable, Insertable, Associations)]
#[primary_key(user_id, group_id)]
#[belongs_to(User)]
#[belongs_to(Group)]
pub struct UserGroupMapping {
    pub user_id: String,
    pub group_id: String
}