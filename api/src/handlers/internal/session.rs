use actix_web::{Json,
    middleware::session::RequestSession,
};
use serde_json::json;

use crate::{Request, Response, encryption, store::users};

#[derive(Deserialize)]
pub struct PostParams {
    email: String,
    password: String,
}

pub fn create((req, params): (Request, Json<PostParams>)) -> Response {
    let secret = &req.state().secret_key;

    match users::find_by_email(&req.state().conn, &params.email) {
        Some(u) => {
            match encryption::verify(secret, &u.password, &params.password) {
                Ok(true) => return match req.session().set("user_id", &u.id) {
                    // TODO get models
                    Ok(_) => Response::Ok().finish(),
                    Err(e) => Response::InternalServerError().body(format!("{}", e)),
                },
                Ok(_) => (), // Fall through to unauthorizedc
                Err(e) => return Response::InternalServerError().body(format!("{}", e)),
            }
        },
        None => {
            // Hash the password to help prevent timing attacks
            let _ = encryption::hash(secret, &params.password);
        },
    }

    Response::Unauthorized().finish()
}

pub fn delete(req: Request) -> Response {
    req.session().clear();
    Response::Ok().finish()
}
