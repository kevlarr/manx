use actix_web::Json;
use actix_web::middleware::session::RequestSession;
use serde_json::json;

use crate::{Request, Response, encryption, store::users};

#[derive(Deserialize)]
pub struct PostParams {
    email: String,
    password: String,
}

pub fn create((req, params): (Request, Json<PostParams>)) -> Response {
    let hash = match encryption::hash(&req.state().secret_key, &params.password) {
        Ok(h) => h,
        Err(e) => return Response::BadRequest().body(format!("{}", e)),
    };

    let new_user = users::new(params.email.clone(), hash);

    match users::create(&req.state().conn, &new_user) {
        Ok(user) => {
            let _ = req.session().set("user_id", user.id);
            Response::Ok().json(json!({ "user": user }))
        },
        Err(e) => Response::BadRequest().body(format!("{}", e)),
    }
}
