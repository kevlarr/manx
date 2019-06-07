use diesel:: prelude::*;
use diesel::pg::PgConnection;

use crate::ApiResult;
use crate::schema;
use crate::models::{Member, Stream, NewStream};

pub fn create(conn: &PgConnection, new_stream: &NewStream) -> ApiResult<Stream> {
    Ok(diesel::insert_into(schema::streams::table)
        .values(new_stream)
        .get_result(conn)?)
}

pub fn for_members(conn: &PgConnection, members: &Vec<Member>) -> ApiResult<Vec<Stream>> {
    use schema::streams::dsl::{streams, id};
    use schema::stream_permissions::dsl::{stream_permissions, stream_id, member_id};

    let member_ids = members.iter().map(|u| u.id);

    let stream_ids = stream_permissions
        .select(stream_id)
        .filter(member_id.eq_any(member_ids));

    Ok(streams.filter(id.eq_any(stream_ids)).load(conn)?)
}
