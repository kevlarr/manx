use diesel:: prelude::*;
use diesel::pg::PgConnection;

use crate::ApiResult;
use crate::schema;
use crate::models::{Comment, NewComment, Topic};

pub fn create(conn: &PgConnection, new_comment: &NewComment) -> ApiResult<Comment> {
    Ok(diesel::insert_into(schema::comments::table)
        .values(new_comment)
        .get_result(conn)?)
}

pub fn for_topics(conn: &PgConnection, topics: &[Topic]) -> ApiResult<Vec<Comment>> {
    use schema::comments::dsl::{comments, topic_id};

    let topic_ids = topics.iter().map(|s| s.id);

    Ok(comments.filter(topic_id.eq_any(topic_ids)).load(conn)?)
}
