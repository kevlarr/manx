use actix_web::Json;

use crate::{Request, Response};

#[derive(Deserialize)]
pub struct CreateParams {
}

pub fn create((_req, _params): (Request, Json<CreateParams>)) -> Response {
    Response::Ok().finish()
}

pub fn update(_req: Request) -> Response {
    Response::Ok().finish()
}

pub fn delete(_req: Request) -> Response {
    Response::Ok().finish()
}
