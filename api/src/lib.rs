#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;

use actix_web::{HttpRequest, HttpResponse};
use diesel::pg::PgConnection;

pub mod encryption;
pub mod handlers;
pub mod models;
pub mod store;

mod schema;

pub struct AppState {
    pub conn: PgConnection,
    pub secret_key: String,
}

pub type Request = HttpRequest<AppState>;
pub type Response = HttpResponse;
