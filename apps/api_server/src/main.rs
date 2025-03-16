mod app_state;
mod schema;
mod services;

use app_state::AppState;
use axum::routing::get;
use diesel::PgConnection;
use once_cell::sync::OnceCell;
use rmpv::Value;
use rs_shared::{
    constants::GameType,
    packets::forza::{parse_forza_packet, schema::InsertF1Data},
    WebsocketPayload,
};
use services::forza::create_f1_data;
use socketioxide::{
    adapter::Adapter,
    extract::{AckSender, Data, SocketRef},
    SocketIo,
};
use socketioxide_redis::{RedisAdapter, RedisAdapterCtr};
use std::{collections::HashSet, env, sync::Arc};
use strum::IntoEnumIterator;
use tokio::sync::Mutex;
use tower::ServiceBuilder;
use tracing_subscriber::FmtSubscriber;

#[macro_use]
extern crate tracing;

static NAMESPACES: OnceCell<HashSet<String>> = OnceCell::new();

async fn on_connect<A: Adapter>(socket: SocketRef<A>, Data(data): Data<Value>) {
    info!(ns = socket.ns(), ?socket.id, "Socket.IO connected");
    socket.emit("auth", &data).ok();

    socket.on("ping", |socket: SocketRef<A>| {
        info!("Pong received for {:?} namespace", socket.ns());
        socket.emit("pong", "🏓").ok();
    });

    socket.on("message", |socket: SocketRef<A>, Data::<Value>(data)| {
        match socket.ns() {
            "/FH4" | "/FH5" | "/FM7" | "/FM8" => {
                if let Some(data) = data.as_slice() {
                    let parsed_msgpack = rmp_serde::from_slice::<WebsocketPayload>(&data).unwrap();

                    let forza_packet = parse_forza_packet(&parsed_msgpack.data);
                }
            }
            _ => {
                warn!("Received message for unknown namespace: {:?}", socket.ns());
            }
        }
        socket.emit("message-back", &data).ok();
    });

    socket.on(
        "message-ack",
        |socket: SocketRef<A>, Data::<Value>(data), ack: AckSender<A>| {
            info!(?data, "Received event");

            // match socket.ns() {
            //     "FH5" => {
            //         if let Some(data) = data.as_slice() {
            //             let parsed_msgpack =
            //                 rmp_serde::from_slice::<WebsocketPayload>(&data).unwrap();

            //             info!("Parsed packet: {:?}", parsed_msgpack);

            //             let forza_packet = parse_forza_packet(&parsed_msgpack.data);

            //             info!("Forza packet: {:?}", forza_packet);
            //         }
            //     }
            //     _ => {
            //         warn!("Received message from unknown namespace: {:?}", socket.ns());
            //     }
            // }
            ack.send(&data).unwrap();
        },
    );
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing::subscriber::set_global_default(FmtSubscriber::default())?;

    info!("Setting up database connection");
    let db_url = env::var("DATABASE_URL").unwrap();

    let manager = deadpool_diesel::postgres::Manager::new(db_url, deadpool_diesel::Runtime::Tokio1);
    let pool = deadpool_diesel::postgres::Pool::builder(manager)
        .build()
        .unwrap();

    info!("Connecting to redis");
    let client = redis::Client::open("redis://127.0.0.1:6380?protocol=resp3")?;
    let adapter = RedisAdapterCtr::new_with_redis(&client).await?;

    let app_state = AppState { pool: pool.clone() };

    info!("Building socket.io layer");
    let (layer, io) = SocketIo::builder()
        // .with_parser(ParserConfig::msgpack())
        .with_adapter::<RedisAdapter<_>>(adapter)
        .max_buffer_size(512)
        .max_payload(2048)
        .with_state(app_state.clone())
        .build_layer();

    info!("Adding namespaces");

    fn get_namespaces() -> &'static HashSet<String> {
        NAMESPACES.get_or_init(|| {
            GameType::iter()
                .map(|game| format!("/{:?}", game))
                .collect()
        })
    }

    for namespace in get_namespaces() {
        let _ = io.ns(namespace, on_connect).await?;
        info!("Namespace added: {}", namespace);
    }

    info!("Setting up routes");
    let app = axum::Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .layer(ServiceBuilder::new().layer(layer));

    info!("Starting server");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3002").await.unwrap();

    info!("Server started on port 3002");
    axum::serve(listener, app).await?;

    Ok(())
}
