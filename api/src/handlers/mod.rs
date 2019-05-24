use actix_web::{Json, Result,
    middleware::session::RequestSession,
};
use serde_json::{json, value::Value as SerdeValue};
use crate::Request;

pub mod internal;

pub fn check(req: Request) -> Result<Json<SerdeValue>> {
    let counter = match req.session().get::<i32>("counter")? {
        Some(c) => c + 1,
        None => 1,
    };

    req.session().set("counter", counter)?;

    Ok(Json(json!({
        "checking in": "all is well!",
        "visit_count": counter,
    })))
}
