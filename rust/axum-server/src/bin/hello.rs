use axum::{Router, extract::Request, routing::get};
use time::OffsetDateTime;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(hello));

    let listener = tokio::net::TcpListener::bind("localhost:3000")
        .await
        .expect("tokio listener failed to bind");

    axum::serve(listener, app)
        .await
        .expect("axum serve failed to serve");
}

async fn hello(r: Request) -> &'static str {
    println!("{} {} {}", OffsetDateTime::now_utc(), r.method(), r.uri());
    "Hello, World!"
}
