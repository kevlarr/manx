use actix_web::Json;
use actix_web::middleware::session::RequestSession;

use crate::{ApiResult, Request};
use crate::encryption;
use crate::models::{User, NewUser};
use crate::stores::users;

#[derive(Deserialize)]
pub struct CreateParams {
    email: String,
    password: String,
}

#[derive(Serialize)]
pub struct Created {
    user: User,
}

pub fn create((req, params): (Request, Json<CreateParams>)) -> ApiResult<Json<Created>> {
    let hash = encryption::hash(&req.state().secret_key, &params.password)?;
    let new_user = NewUser::new(params.email.clone(), hash);
    let user = users::create(&req.state().conn, &new_user)?;

    // Failing to set session should not error, since user is now created.
    let _ = req.session().set("user_id", user.id);

    Ok(Json(Created { user }))
}
