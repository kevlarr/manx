use diesel:: prelude::*;
use diesel::pg::PgConnection;

use crate::models::{OrganizationUser, User};
use crate::store::{StoreResult, StoreError};

pub fn for_user(conn: &PgConnection, user: &User) -> StoreResult<Vec<OrganizationUser>> {
    OrganizationUser::belonging_to(user)
        .load(conn)
        .map_err(StoreError::from_err)
}
