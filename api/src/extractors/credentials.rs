use actix_web::{FromRequest, HttpRequest};
use actix_web::middleware::session::RequestSession;

use crate::{AppState, ApiResult};
use crate::error::ApiError;
use crate::models::User;
use crate::stores::users;

pub struct Credentials {
    pub user: User,
}

impl FromRequest<AppState> for Credentials {
    type Config = ();
    type Result = ApiResult<Credentials>;

    fn from_request(req: &HttpRequest<AppState>, _cfg: &Self::Config) -> Self::Result {
        let conn = &req.state().conn;

        if let Ok(Some(user_id)) = req.session().get::<i32>("user_id") {
            if let Ok(Some(user)) = users::find(conn, user_id) {
                return Ok(Credentials { user });
            }
        }

        Err(ApiError::Unauthorized)
    }
}
