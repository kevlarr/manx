use chrono::Utc;
use diesel:: prelude::*;
use diesel::pg::PgConnection;

use crate::schema;
use crate::models::{User, NewUser};
use super::StoreError;

type StoreResult<T> = std::result::Result<T, StoreError>;

pub fn new(email: String, password: String) -> NewUser {
    NewUser { created: Utc::now(), email, password }
}

pub fn create(conn: &PgConnection, new_user: &NewUser) -> StoreResult<User> {
    diesel::insert_into(schema::users::table)
        .values(new_user)
        .get_result(conn)
        .map_err(|e| StoreError::new(format!("{}", e)))
}

pub fn find_by_email(conn: &PgConnection, email: &str) -> Option<User> {
    use schema::users::dsl::{users, email as email_};

    users.filter(email_.eq(email)).first(conn).ok()
}
