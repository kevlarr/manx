use actix_web::{
    http::Method,
    server,
    App,
    HttpRequest,
    HttpResponse
};
use diesel::{pg::PgConnection, prelude::*};
use dotenv::dotenv;
use std::env;

struct AppState {
    conn: PgConnection,
}

type Request = HttpRequest<AppState>;
type Response = HttpResponse;

fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("Must set DATABASE_URL");

    let create_app = move || {
        let conn = PgConnection::establish(&database_url)
            .expect("Error connecting to database");

        App::with_state(AppState { conn })
            .route("hello", Method::GET, hello)
    };

    let port = env::var("PORT").unwrap_or("1337".to_string());
    let addr = format!("127.0.0.1:{}", port);
    println!("Now listening on {}", addr);

    server::new(create_app)
        .bind(addr)
        .unwrap()
        .run();
}

fn hello(_req: Request) -> Response {
    Response::Ok().body("
        <!doctype html>
        <html>
            <head><title>Hello, Manx!</title></head>
            <body><h1>Hello, Manx!</h1></body>
        </html>
    ")
}
