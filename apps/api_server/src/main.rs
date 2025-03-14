mod schema;

use axum::routing::get;
use diesel::prelude::*;
use once_cell::sync::OnceCell;
use rmpv::Value;
use rs_shared::constants::GameType;
use socketioxide::{
    extract::{AckSender, Data, SocketRef},
    SocketIo,
};
use std::{collections::HashSet, env};
use strum::IntoEnumIterator;
use tracing::info;
use tracing_subscriber::FmtSubscriber;

static NAMESPACES: OnceCell<HashSet<String>> = OnceCell::new();

fn on_connect(socket: SocketRef, Data(data): Data<Value>) {
    info!(ns = socket.ns(), ?socket.id, "Socket.IO connected");
    socket.emit("auth", &data).ok();

    socket.on("ping", |socket: SocketRef| {
        socket.emit("pong", "🏓").ok();
    });

    socket.on("message", |socket: SocketRef, Data::<Value>(data)| {
        // info!(?data, "Received event:");
        socket.emit("message-back", &data).ok();
    });

    socket.on("message-ack", |Data::<Value>(data), ack: AckSender| {
        info!(?data, "Received event");
        ack.send(&data).ok();
    });
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing::subscriber::set_global_default(FmtSubscriber::default())?;

    let (layer, io) = SocketIo::new_layer();

    // init namespaces
    io.ns("/", on_connect);

    fn get_namespaces() -> &'static HashSet<String> {
        NAMESPACES.get_or_init(|| {
            GameType::iter()
                .map(|game| format!("/{:?}", game))
                .collect()
        })
    }

    for namespace in get_namespaces() {
        io.ns(namespace, on_connect);
        info!("Namespace added: {}", namespace);
    }

    let db_url = env::var("DATABASE_URL").unwrap();

    // set up connection pool
    let manager = deadpool_diesel::postgres::Manager::new(db_url, deadpool_diesel::Runtime::Tokio1);
    let pool = deadpool_diesel::postgres::Pool::builder(manager)
        .build()
        .unwrap();

    // init routes
    let app = axum::Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .layer(layer);

    info!("Starting server");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3002").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
