use diesel:: prelude::*;
use diesel::pg::PgConnection;

use crate::ApiResult;
use crate::schema;
use crate::models::{User, NewUser};

pub fn create(conn: &PgConnection, new_user: &NewUser) -> ApiResult<User> {
    Ok(diesel::insert_into(schema::users::table)
        .values(new_user)
        .get_result(conn)?)
}

pub fn find(conn: &PgConnection, user_id: i32) -> ApiResult<Option<User>> {
    Ok(schema::users::dsl::users
        .find(user_id)
        .get_result(conn)
        .optional()?)
}

pub fn find_by_email(conn: &PgConnection, email: &str) -> ApiResult<Option<User>> {
    use schema::users::dsl::{users, email as email_};

    Ok(users
        .filter(email_.eq(email))
        .first(conn)
        .optional()?)
}
