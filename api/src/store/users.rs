use chrono::Utc;
use diesel:: prelude::*;
use diesel::pg::PgConnection;

use crate::schema;
use crate::models::{User, NewUser};
use super::{StoreResult, StoreError};

pub fn new(email: String, password: String) -> NewUser {
    NewUser { created: Utc::now(), email, password }
}

pub fn create(conn: &PgConnection, new_user: &NewUser) -> StoreResult<User> {
    diesel::insert_into(schema::users::table)
        .values(new_user)
        .get_result(conn)
        .map_err(|e| StoreError::new(format!("{}", e)))
}

pub fn find(conn: &PgConnection, user_id: i32) -> StoreResult<User> {
    schema::users::dsl::users
        .find(user_id)
        .get_result(conn)
        .map_err(|e| StoreError::new(format!("{}", e)))
}

pub fn find_by_email(conn: &PgConnection, email: &str) -> StoreResult<User> {
    use schema::users::dsl::{users, email as email_};

    users
        .filter(email_.eq(email))
        .first(conn)
        .map_err(|e| StoreError::new(format!("{}", e)))
}
