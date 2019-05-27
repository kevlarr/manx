use chrono::Utc;
use diesel:: prelude::*;
use diesel::pg::PgConnection;

use crate::ApiResult;
use crate::error::ApiError;
use crate::schema;
use crate::models::{User, NewUser};

pub fn new(email: String, password: String) -> NewUser {
    NewUser { created: Utc::now(), email, password }
}

pub fn create(conn: &PgConnection, new_user: &NewUser) -> ApiResult<User> {
    diesel::insert_into(schema::users::table)
        .values(new_user)
        .get_result(conn)
        .map_err(ApiError::from)
}

pub fn find(conn: &PgConnection, user_id: i32) -> ApiResult<Option<User>> {
    schema::users::dsl::users
        .find(user_id)
        .get_result(conn)
        .optional()
        .map_err(ApiError::from)
}

pub fn find_by_email(conn: &PgConnection, email: &str) -> ApiResult<Option<User>> {
    use schema::users::dsl::{users, email as email_};

    users
        .filter(email_.eq(email))
        .first(conn)
        .optional()
        .map_err(ApiError::from)
}
