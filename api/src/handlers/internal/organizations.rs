use actix_web::Json;

use crate::{Request, Response};

#[derive(Deserialize)]
pub struct PostParams {
}

pub fn create((_req, _params): (Request, Json<PostParams>)) -> Response {
    Response::Ok().finish()
}

pub fn list(_req: Request) -> Response {
    Response::Ok().finish()
}

pub fn delete(_req: Request) -> Response {
    Response::Ok().finish()
}
