use crate::init::init;
use anyhow::Result;
use api::*;
use axum::http::header::CONTENT_TYPE;
use axum::http::Method;
use axum::routing::get;
use axum::{routing::post, Router};
use tower_http::cors::CorsLayer;
use websocket::server::ws_handler;
use websocket::spawn_client::spawn_ws_client;

mod api;
mod fetcher;
mod global;
mod init;
mod json;
mod notification;
mod unsanitizer;
mod user;
mod validate;
mod websocket;

#[tokio::main]
async fn main() -> Result<()> {
    init()?;

    let init::Data {
        listen,
        token,
        ..
    } = json::read_json("data.json")?;

    if !token.is_empty() {
        tokio::join!(spawn_ws_client(), internal::init_var::init_var(&token)).1?;
    }
    drop(token);

    let app = Router::new()
        .route("/", get(ws_handler))
        .route(
            "/reboot",
            post(move |auth: String| async move {
                drop(validate::validate(&auth)?);
                spawn_ws_client().await;
                Ok(true)
            }),
        )
        .route("/auth", post(api_auth))
        .route("/user", post(api_user))
        .route("/friends", post(api_friends))
        .route("/twofactor", post(api_twofactor))
        .route("/group", post(api_group))
        .route("/instance", post(api_instance))
        .route("/world", post(api_world))
        .layer(
            CorsLayer::new()
                .allow_origin(tower_http::cors::Any)
                .allow_methods([Method::POST, Method::OPTIONS])
                .allow_headers([CONTENT_TYPE]),
        );

    let listener = tokio::net::TcpListener::bind(&listen).await?;

    axum::serve(listener, app).await?;

    Ok(())
}
