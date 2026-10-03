//! This is a simple wrapper around a 3rd party smtp server
//! to expose a POST endpoint for other apps to send emails through.
use axum::{
    Json, Router,
    extract::State,
    http::{HeaderMap, StatusCode},
    routing::{get, post},
};
use lettre::{
    Message, SmtpTransport, Transport,
    message::{Mailbox, header::ContentType},
};
use std::{env, net::SocketAddr, sync::LazyLock};
use tower_http::cors::Any;

struct Config {
    // like "smtps://user:pass@smtp.mail.com"
    mailer_url: String,
    // Can be "NAME <email@addr.com>" or "email@addr.com"
    mail_from: Mailbox,
}

impl Config {
    fn from_env() -> Self {
        let get = |key: &str| env::var(key).unwrap_or_else(|_| panic!("{} undefined", key));
        Self {
            mailer_url: get("SMTP_URL"),
            mail_from: get("MAIL_FROM")
                .parse::<Mailbox>()
                .expect("MAIL_FROM invalid"),
        }
    }
}

static CONFIG: LazyLock<Config> = LazyLock::new(Config::from_env);

static AUTH_TOKEN: LazyLock<String> = LazyLock::new(|| {
    env::var("AUTH_TOKEN").unwrap_or_else(|_| {
        eprintln!("warning: no authorization token found in environment");
        String::new()
    })
});

#[derive(Clone)]
struct AppState {
    smtp: SmtpTransport,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt().init(); // TODO: Add atomic Id

    let host = env::var("HOST")
        .unwrap_or_else(|_| "127.0.0.1".into())
        .parse()?;
    let port = env::var("PORT").unwrap_or_else(|_| "3000".into()).parse()?;
    let addr = SocketAddr::new(host, port);

    let smtp = SmtpTransport::from_url(&CONFIG.mailer_url)?.build();

    log::info!("server listening on {}", addr);
    Ok(axum::serve(
        tokio::net::TcpListener::bind(addr).await?,
        Router::new()
            .route("/", post(handler))
            .route("/health", get(StatusCode::OK))
            .layer(
                tower_http::cors::CorsLayer::new()
                    .allow_origin(Any)
                    .allow_methods(Any)
                    .allow_headers(Any),
            )
            .with_state(AppState { smtp }),
    )
    .await?)
}

#[derive(serde::Deserialize)]
struct ReqBody {
    to: String,
    subject: String,
    contents: String,
}

#[axum::debug_handler]
async fn handler(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(body): Json<ReqBody>,
) -> Result<(), StatusCode> {
    log::info!("received send request for {}", body.to);

    let recv_token = headers
        .get("authorization")
        .and_then(|f| f.to_str().ok())
        .unwrap_or("");

    // TODO: constant time check
    if recv_token != *AUTH_TOKEN {
        log::error!("auth check did not succeed");
        return Err(StatusCode::UNAUTHORIZED);
    }

    let to = body.to.parse::<Mailbox>().map_err(|_| {
        log::error!("invalid to addr {}", body.to);
        StatusCode::BAD_REQUEST
    })?;

    let email = Message::builder()
        .header(ContentType::TEXT_PLAIN)
        .from(CONFIG.mail_from.clone())
        .to(to)
        .subject(body.subject)
        .body(body.contents)
        .map_err(|e| {
            log::error!("unable to build email: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    state.smtp.send(&email).map_err(|e| {
        log::error!("unable to send email: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(())
}
