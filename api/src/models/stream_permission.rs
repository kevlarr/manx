use chrono::{DateTime, Utc};

use crate::schema::stream_permissions;

#[derive(Debug, Queryable, Serialize)]
pub struct StreamPermission {
    pub id: i32,

    #[serde(rename = "memberId")]
    pub member_id: i32,

    #[serde(rename = "streamId")]
    pub stream_id: i32,

    #[serde(skip_serializing)]
    pub created: DateTime<Utc>,
}

#[derive(Insertable)]
#[table_name="stream_permissions"]
pub struct NewStreamPermission {
    pub member_id: i32,
    pub stream_id: i32,
    pub created: DateTime<Utc>,
}

impl NewStreamPermission {
    pub fn new(member_id: i32, stream_id: i32) -> Self {
        NewStreamPermission { member_id, stream_id, created: Utc::now() }
    }
}
