use chrono::{DateTime, Utc};
use super::schema::{
    organizations,
    organization_users,
    streams,
    stream_users,
    stream_topics,
    users,
};


#[derive(Debug, Queryable, Serialize)]
pub struct User {
    pub id: i32,
    #[serde(skip_serializing)]
    pub created: DateTime<Utc>,
    pub email: String,
    #[serde(skip_serializing)]
    pub password: String,
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser {
    pub email: String,
    pub password: String,
    pub created: DateTime<Utc>,
}

#[derive(Debug, Queryable, Serialize)]
pub struct Organization {
    pub id: i32,
    pub created: DateTime<Utc>,
    pub title: String,
    pub uri_key: String,
    pub user_id: i32,
}

#[derive(Insertable)]
#[table_name="organizations"]
pub struct NewOrganization {
    pub created: DateTime<Utc>,
    pub title: String,
    pub uri_key: String,
    pub user_id: i32,
}

#[derive(Debug, Queryable, Serialize)]
pub struct OrganizationUser {
    pub id: i32,
    pub created: DateTime<Utc>,
    pub name: String,
    pub user_id: i32,
    pub organization_id: i32,
}

#[derive(Insertable)]
#[table_name="organization_users"]
pub struct NewOrganizationUser {
    pub created: DateTime<Utc>,
    pub name: String,
    pub user_id: i32,
    pub organization_id: i32,
}

#[derive(Debug, Queryable, Serialize)]
pub struct Stream {
    pub id: i32,
    pub created: DateTime<Utc>,
    pub name: String,
    pub uri_key: String,
    pub global: bool,
    pub organization_id: i32,
    pub organization_user_id: i32,
    pub parent_id: Option<i32>,
}

#[derive(Insertable)]
#[table_name="streams"]
pub struct NewStream {
    pub created: DateTime<Utc>,
    pub name: String,
    pub uri_key: String,
    pub global: bool,
    pub organization_id: i32,
    pub organization_user_id: i32,
    pub parent_id: Option<i32>,
}

#[derive(Debug, Queryable, Serialize)]
pub struct StreamUser {
    pub id: i32,
    pub created: DateTime<Utc>,
    pub organization_user_id: i32,
    pub stream_id: i32,
}

#[derive(Insertable)]
#[table_name="stream_users"]
pub struct NewStreamUser {
    pub id: i32,
    pub created: DateTime<Utc>,
    pub organization_user_id: i32,
    pub stream_id: i32,
}

#[derive(Debug, Queryable, Serialize)]
pub struct StreamTopic {
    pub id: i32,
    pub created: DateTime<Utc>,
    pub raw: String,
    pub rendered: String,
    pub organization_user_id: i32,
    pub stream_id: i32,
}

#[derive(Insertable)]
#[table_name="stream_topics"]
pub struct NewStreamTopic {
    pub created: DateTime<Utc>,
    pub raw: String,
    pub rendered: String,
    pub organization_user_id: i32,
    pub stream_id: i32,
}
