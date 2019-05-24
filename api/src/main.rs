use actix_web::{App, server,
    http::Method,
    middleware::{Logger,
        session::{SessionStorage, CookieSessionBackend},
    },
};
use diesel::{pg::PgConnection, prelude::*};
use dotenv::dotenv;
use std::env;

use manx::{AppState,
    handlers::{self, internal},
};

fn create() -> App<AppState> {
    let protoc = env::var("PROTOCOL").unwrap_or_else(|_| "https".to_string());
    let db_url = env::var("DATABASE_URL").expect("Must set DATABASE_URL");
    let domain = env::var("DOMAIN").expect("Must set DOMAIN");
    let secret = env::var("SECRET_KEY").expect("Must set SECRET_KEY");

    let conn = PgConnection::establish(&db_url).expect("Error connecting to database");
    let state = AppState { conn, secret_key: secret.clone() };

    let session_storage = SessionStorage::new(
        CookieSessionBackend::private(secret.as_bytes())
            .name("manx")
            .domain(domain)
            .secure(&protoc == "https")
            .http_only(false)
    );

    App::with_state(state)
        .middleware(Logger::default())
        .middleware(session_storage)
        .prefix("api")
        .route("check", Method::GET, handlers::check)
        .scope("internal", |internal| internal
            .resource("session", |r| {
                r.method(Method::POST).with(internal::session::create);
                r.method(Method::DELETE).with(internal::session::delete);
            })
            .nested("users", |users| users
                .resource("", |r| {
                    r.method(Method::POST).with(internal::users::create);
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
