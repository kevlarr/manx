use chrono::{DateTime, Utc};

use crate::encryption;
use crate::schema::streams;

#[derive(Debug, Queryable, Serialize)]
pub struct Stream {
    pub id: i32,

    #[serde(rename = "teamId")]
    pub team_id: i32,

    #[serde(rename = "memberId")]
    pub member_id: i32,

    #[serde(rename = "parentId")]
    pub parent_id: Option<i32>,

    pub created: DateTime<Utc>,
    pub title: String,
    pub key: String,
}

#[derive(Insertable)]
#[table_name="streams"]
pub struct NewStream {
    pub team_id: i32,
    pub member_id: i32,
    pub parent_id: Option<i32>,
    pub created: DateTime<Utc>,
    pub title: String,
    pub key: String,
}

impl NewStream {
    pub fn new(team_id: i32, member_id: i32, title: String) -> Self {
        NewStream {
            team_id,
            member_id: member_id,
            parent_id: None,
            created: Utc::now(),
            title,
            key: encryption::rand_key(),
        }
    }
}
