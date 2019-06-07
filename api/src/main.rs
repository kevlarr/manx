use actix_web::{App, server};
use actix_web::http::Method;
use actix_web::middleware::Logger;
use actix_web::middleware::session::{SessionStorage, CookieSessionBackend};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use manx::AppState;
use manx::handlers::check as health_check;
use manx::handlers::internal::{
    users::{
        create as create_user,
    },
    session::{
        create as create_session,
        delete as delete_session,
    },
    teams::{
        list as list_teams,
        create as create_team,
    },
    streams::{
        create as create_stream,
        update as update_stream,
        delete as delete_stream,
    },
    topics::{
        create as create_topic,
    },
    comments::{
        create as create_comment,
    },
};

fn create() -> App<AppState> {
    let protoc = env::var("PROTOCOL").unwrap_or_else(|_| "https".to_string());
    let db_url = env::var("DATABASE_URL").expect("Must set DATABASE_URL");
    let domain = env::var("DOMAIN").expect("Must set DOMAIN");
    let secret = env::var("SECRET_KEY").expect("Must set SECRET_KEY");

    let conn = PgConnection::establish(&db_url).expect("Error connecting to database");
    let state = AppState { conn, secret_key: secret.clone() };

    let session_storage = SessionStorage::new(
        CookieSessionBackend::signed(secret.as_bytes())
            .name("manx")
            .domain(domain)
            .secure(&protoc == "https")
            .http_only(false)
    );

    App::with_state(state)
        .middleware(Logger::default())
        .middleware(session_storage)
        .prefix("api")
        .route("check", Method::GET, health_check)
        .scope("internal", |api| api
            .nested("teams", |teams| teams
                .resource("", |r| {
                    r.method(Method::GET).with(list_teams);
                    r.method(Method::POST).with(create_team);
                })
                .nested("{team}", |team| team
                    .resource("streams", |r| {
                        r.method(Method::POST).with(create_stream);
                    })
                )
            )
            .resource("session", |r| {
                r.method(Method::POST).with(create_session);
                r.method(Method::DELETE).with(delete_session);
            })
            .nested("streams", |streams| streams
                .nested("{stream}", |stream| stream
                    .resource("", |r| {
                        r.method(Method::PATCH).with(update_stream);
                        r.method(Method::DELETE).with(delete_stream);
                    })
                    .resource("topics", |r| {
                        r.method(Method::POST).with(create_topic);
                    })
                )
            )
            .nested("topics", |topics| topics
                .nested("{topic}", |topic| topic
                    .resource("comments", |r| {
                        r.method(Method::POST).with(create_comment);
                    })
                )
            )
            .nested("users", |users| users
                .resource("", |r| {
                    r.method(Method::POST).with(create_user);
                })
            )
        )
}

fn main() {
    dotenv().ok();

    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let port = env::var("PORT").unwrap_or_else(|_| "1337".to_string());
    let addr = format!("127.0.0.1:{}", port);

    println!("Now listening on {}", addr);

    server::new(create).bind(addr).unwrap().run();
}
