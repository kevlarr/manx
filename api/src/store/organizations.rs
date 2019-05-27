use diesel:: prelude::*;
use diesel::pg::PgConnection;

use crate::ApiResult;
use crate::error::ApiError;
use crate::schema;
use crate::models::{Organization, OrganizationUser};

pub fn for_organization_users(conn: &PgConnection, users: &Vec<OrganizationUser>) -> ApiResult<Vec<Organization>> {
    use schema::organizations::dsl::{organizations, id};

    let ids = users.iter().map(|u| u.organization_id);

    organizations
        .filter(id.eq_any(ids))
        .load(conn)
        .map_err(ApiError::from)
}
