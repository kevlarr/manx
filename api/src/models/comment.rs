use chrono::{DateTime, Utc};

use crate::markdown;
use crate::schema::comments;

#[derive(Debug, Queryable, Serialize)]
pub struct Comment {
    pub id: i32,

    #[serde(rename = "memberId")]
    pub member_id: i32,

    #[serde(rename = "topicId")]
    pub topic_id: i32,

    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
    pub raw: String,
    pub rendered: String,
}

#[derive(Insertable)]
#[table_name="comments"]
pub struct NewComment {
    pub member_id: i32,
    pub topic_id: i32,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
    pub raw: String,
    pub rendered: String,
}

impl NewComment {
    pub fn new(member_id: i32, topic_id: i32, content: String) -> Self {
        let rendered = markdown::render(&content);
        let now = Utc::now();

        NewComment {
            member_id,
            topic_id,
            created: now,
            updated: now,
            raw: content,
            rendered,
        }
    }
}
