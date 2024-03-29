use actix_web::{Json, Path};

use crate::Request;
use crate::ApiResult;
use crate::extractors::Credentials;
use crate::models::{Topic, NewTopic};
use crate::stores::{members, topics};

#[derive(Deserialize)]
pub struct CreateParams {
    raw: String,
}

#[derive(Serialize)]
pub struct Created {
    topic: Topic,
}

pub fn create(
    (req, cred, path, params): (Request, Credentials, Path<(i32,)>, Json<CreateParams>)
) -> ApiResult<Json<Created>>
{
    let conn = &req.state().conn;
    let stream_id = path.0;

    let member_id = members::id_from_stream(conn, cred.user.id, stream_id)?;

    let new_topic = NewTopic::new(member_id, stream_id, params.raw.clone());
    let topic = topics::create(conn, &new_topic)?;

    Ok(Json(Created { topic }))
}
