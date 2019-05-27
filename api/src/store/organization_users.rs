use diesel:: prelude::*;
use diesel::pg::PgConnection;

use crate::ApiResult;
use crate::error::ApiError;
use crate::models::{OrganizationUser, User};

pub fn for_user(conn: &PgConnection, user: &User) -> ApiResult<Vec<OrganizationUser>> {
    OrganizationUser::belonging_to(user)
        .load(conn)
        .map_err(ApiError::from)
}
