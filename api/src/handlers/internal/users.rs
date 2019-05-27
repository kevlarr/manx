use actix_web::Json;
use actix_web::middleware::session::RequestSession;
use serde_json::json;
use serde_json::value::Value;

use crate::{ApiResult, Request};
use crate::encryption;
use crate::store::users;

#[derive(Deserialize)]
pub struct PostParams {
    email: String,
    password: String,
}

pub fn create((req, params): (Request, Json<PostParams>)) -> ApiResult<Json<Value>> {
    let hash = encryption::hash(&req.state().secret_key, &params.password)?;
    let new_user = users::new(params.email.clone(), hash);

    let user = users::create(&req.state().conn, &new_user)?;

    // Failing to set session should not error, since user is now created.
    let _ = req.session().set("user_id", user.id);

    Ok(Json(json!({ "user": user })))
}
