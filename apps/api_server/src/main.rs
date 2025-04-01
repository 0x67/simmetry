mod app_config;
mod app_state;
mod handlers;
mod services;
mod setup;
mod types;

use app_config::APP_CONFIG;
use axum::routing::get;
use setup::{setup_app, setup_socketio};
use tower::ServiceBuilder;
use tracing_subscriber::FmtSubscriber;

#[macro_use]
extern crate tracing;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing::subscriber::set_global_default(FmtSubscriber::default())?;

    let (app_state, adapter) = setup_app().await?;

    let (socketio_layer, _) = setup_socketio(app_state.clone(), adapter).await?;

    info!("Setting up routes");
    let app = axum::Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .layer(ServiceBuilder::new().layer(socketio_layer))
        .with_state(app_state);

    info!("Starting server");

    let port = APP_CONFIG.port.clone();
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();

    info!("Server started on port {}", port);
    axum::serve(listener, app).await?;

    Ok(())
}
