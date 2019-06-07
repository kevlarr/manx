use diesel:: prelude::*;
use diesel::pg::PgConnection;

use crate::ApiResult;
use crate::schema;
use crate::models::{StreamPermission, NewStreamPermission};

pub fn create(conn: &PgConnection, new_permission: &NewStreamPermission) -> ApiResult<StreamPermission> {
    Ok(diesel::insert_into(schema::stream_permissions::table)
        .values(new_permission)
        .get_result(conn)?)
}
