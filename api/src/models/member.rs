use chrono::{DateTime, Utc};

use crate::models::{Team, User};
use crate::schema::members;

#[derive(Associations, Debug, Identifiable, Queryable, Serialize)]
#[belongs_to(Team)]
#[belongs_to(User)]
pub struct Member {
    pub id: i32,

    #[serde(rename = "teamId")]
    pub team_id: i32,

    #[serde(skip_serializing)]
    pub user_id: Option<i32>,

    pub created: DateTime<Utc>,
    pub name: String,
}

#[derive(Insertable)]
#[table_name="members"]
pub struct NewMember {
    pub team_id: i32,
    pub user_id: i32,
    pub created: DateTime<Utc>,
    pub name: String,
}

impl NewMember {
    pub fn new(user_id: i32, team_id: i32, name: String) -> Self {
        NewMember { team_id, user_id, created: Utc::now(), name }
    }
}
