use chrono::{DateTime, Utc};

use crate::schema::users;

#[derive(Debug, Identifiable, Queryable, Serialize)]
pub struct User {
    pub id: i32,
    pub created: DateTime<Utc>,
    pub email: String,

    #[serde(skip_serializing)]
    pub password: String,
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser {
    pub created: DateTime<Utc>,
    pub email: String,
    pub password: String,
}

impl NewUser {
    pub fn new(email: String, password: String) -> NewUser {
        NewUser { created: Utc::now(), email, password }
    }
}
