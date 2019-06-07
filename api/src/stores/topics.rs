use diesel:: prelude::*;
use diesel::pg::PgConnection;

use crate::ApiResult;
use crate::schema;
use crate::models::{Topic, NewTopic, Stream};

pub fn create(conn: &PgConnection, new_topic: &NewTopic) -> ApiResult<Topic> {
    Ok(diesel::insert_into(schema::topics::table)
        .values(new_topic)
        .get_result(conn)?)
}

pub fn for_streams(conn: &PgConnection, streams: &[Stream]) -> ApiResult<Vec<Topic>> {
    use schema::topics::dsl::{topics, stream_id};

    let stream_ids = streams.iter().map(|s| s.id);

    Ok(topics.filter(stream_id.eq_any(stream_ids)).load(conn)?)
}
