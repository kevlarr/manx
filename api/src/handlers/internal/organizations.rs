use actix_web::Json;
use actix_web::middleware::session::RequestSession;
use serde_json::json;

use crate::{Request, Response};
use crate::models::{Organization, OrganizationUser, Stream};
use crate::store::{organizations, organization_users, users};

#[derive(Deserialize)]
pub struct PostParams {
}

pub fn create((_req, _params): (Request, Json<PostParams>)) -> Response {
    Response::Ok().finish()
}

// TODO
//pub fn list((req, credentials): (Request, Credentials)) -> Response {
pub fn list(req: Request) -> Response {
    let conn = &req.state().conn;

    let user_id = match req.session().get::<i32>("user_id") {
        Ok(val) => match val {
            Some(uid) => uid,
            None => return Response::Unauthorized().finish(),
        },
        Err(_) => return Response::Unauthorized().finish(),
    };

    let user = match users::find(conn, user_id) {
        Ok(u) => u,
        Err(_) => return Response::Unauthorized().finish(),
    };

    let orgusers = match organization_users::for_user(conn, &user) {
        Ok(users) => users,
        Err(e) => return Response::BadRequest().body(format!("{}", e)),
    };

    let orgs = match organizations::for_organization_users(conn, &orgusers) {
        Ok(o) => o,
        Err(e) => return Response::BadRequest().body(format!("{}", e)),
    };


    Response::Ok().json(json!({
        "organizations": orgs,
        "organizationUsers": orgusers,
        //"streams": None,
    }))
}

pub fn delete(_req: Request) -> Response {
    Response::Ok().finish()
}
