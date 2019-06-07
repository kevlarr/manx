use actix_web::{Json, Path};

use crate::Request;
use crate::ApiResult;
use crate::extractors::Credentials;
use crate::models::{Comment, NewComment};
use crate::stores::{members, comments};

#[derive(Deserialize)]
pub struct CreateParams {
    raw: String,
}

#[derive(Serialize)]
pub struct Created {
    comment: Comment,
}

pub fn create(
    (req, cred, path, params): (Request, Credentials, Path<(i32,)>, Json<CreateParams>)
) -> ApiResult<Json<Created>>
{
    let conn = &req.state().conn;
    let topic_id = path.0;

    let member_id = members::id_from_topic(conn, cred.user.id, topic_id)?;

    let new_comment = NewComment::new(member_id, topic_id, params.raw.clone());
    let comment = comments::create(conn, &new_comment)?;

    Ok(Json(Created { comment }))
}
