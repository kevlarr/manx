use chrono::{DateTime, Utc};

use crate::encryption;
use crate::schema::teams;

#[derive(Debug, Queryable, Serialize)]
pub struct Team {
    pub id: i32,

    #[serde(skip_serializing)]
    pub owner_id: i32,

    pub created: DateTime<Utc>,
    pub title: String,
    pub key: String,
}

#[derive(Insertable)]
#[table_name="teams"]
pub struct NewTeam {
    pub owner_id: i32,
    pub created: DateTime<Utc>,
    pub title: String,
    pub key: String,
}

impl NewTeam {
    pub fn new(owner_id: i32, title: String) -> Self {
        NewTeam {
            owner_id,
            created: Utc::now(),
            title,
            key: encryption::rand_key(),
        }
    }
}
