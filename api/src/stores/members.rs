use diesel:: prelude::*;
use diesel::pg::PgConnection;

use crate::ApiResult;
use crate::models::{Member, NewMember, User};
use crate::schema;

pub fn create(conn: &PgConnection, new_member: &NewMember) -> ApiResult<Member> {
    Ok(diesel::insert_into(schema::members::table)
        .values(new_member)
        .get_result(conn)?)
}

pub fn for_user(conn: &PgConnection, user: &User) -> ApiResult<Vec<Member>> {
    Ok(Member::belonging_to(user).load(conn)?)
}

pub fn id_from_stream(conn: &PgConnection, user_id: i32, stream_id: i32) -> ApiResult<i32> {
    use schema::{members, streams, stream_permissions as perms};

    Ok(streams::table
        .inner_join(perms::table)
        .inner_join(members::table)
        .filter(members::user_id.eq(user_id))
        .filter(streams::id.eq(stream_id))
        .select(members::id)
        .first(conn)?)
}

pub fn id_from_topic(conn: &PgConnection, user_id: i32, topic_id: i32) -> ApiResult<i32> {
    use schema::{members, streams, stream_permissions as perms, topics};

    Ok(topics::table
        .inner_join(streams::table)
        .inner_join(perms::table.on(perms::stream_id.eq(streams::id)))
        .inner_join(members::table)
        .filter(members::user_id.eq(user_id))
        .filter(topics::id.eq(topic_id))
        .select(members::id)
        .first(conn)?)
}
