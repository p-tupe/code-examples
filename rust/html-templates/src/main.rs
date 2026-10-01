//! This code contains a minimal html template code
//! Refer https://askama.rs/en/stable/
use std::error::Error;

use askama::Template;
use axum::{Router, response::IntoResponse, routing::get};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let router = Router::new().route("/", get(root));
    let listener = tokio::net::TcpListener::bind("localhost:8080").await?;
    Ok(axum::serve(listener, router).await?)
}

#[derive(Template)]
#[template(path = "root.html")]
struct RootTmpl {
    name: String,
}

async fn root() -> impl IntoResponse {
    RootTmpl {
        name: "Pat".to_string(),
    }
    .render()
    .unwrap_or("some error happened".to_string())
}
