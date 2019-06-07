use chrono::{DateTime, Utc};

use crate::{encryption, markdown};
use crate::schema::topics;

#[derive(Debug, Queryable, Serialize)]
pub struct Topic {
    pub id: i32,

    #[serde(rename = "memberId")]
    pub member_id: i32,

    #[serde(rename = "streamId")]
    pub stream_id: i32,

    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
    pub raw: String,
    pub rendered: String,
    pub key: String,
}

#[derive(Insertable)]
#[table_name="topics"]
pub struct NewTopic {
    pub member_id: i32,
    pub stream_id: i32,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
    pub raw: String,
    pub rendered: String,
    pub key: String,
}

impl NewTopic {
    pub fn new(member_id: i32, stream_id: i32, content: String) -> Self {
        let rendered = markdown::render(&content);
        let now = Utc::now();

        NewTopic {
            member_id,
            stream_id,
            created: now,
            updated: now,
            raw: content,
            rendered,
            key: encryption::rand_key(),
        }
    }
}
