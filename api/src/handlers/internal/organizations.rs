use actix_web::Json;
use serde_json::json;
use serde_json::value::Value;

use crate::{ApiResult, Request, Response};
use crate::extractors::Credentials;
use crate::store::{organizations, organization_users};

#[derive(Deserialize)]
pub struct PostParams {
}

pub fn create((_req, _params): (Request, Json<PostParams>)) -> Response {
    Response::Ok().finish()
}

pub fn list((req, credentials): (Request, Credentials)) -> ApiResult<Json<Value>> {
    let conn = &req.state().conn;

    let orgusers = organization_users::for_user(conn, &credentials.user)?;
    let orgs = organizations::for_organization_users(conn, &orgusers)?;

    Ok(Json(json!({
        "organizations": orgs,
        "organizationUsers": orgusers,
        //"streams": None,
    })))
}

pub fn delete(_req: Request) -> Response {
    Response::Ok().finish()
}
