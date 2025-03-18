mod app_state;
mod handlers;
mod services;

use app_state::AppState;
use axum::routing::get;
use crossbeam_channel::unbounded;
use handlers::ws::socket_on_connect;
use once_cell::sync::OnceCell;
use rs_shared::{constants::GameType, database::models::forza::ForzaData};
use services::forza::ForzaService;
use socketioxide::SocketIo;
use socketioxide_redis::{RedisAdapter, RedisAdapterCtr};
use std::{collections::HashSet, env, sync::Arc};
use strum::IntoEnumIterator;
use tower::ServiceBuilder;
use tracing_subscriber::FmtSubscriber;

#[macro_use]
extern crate tracing;

static NAMESPACES: OnceCell<HashSet<String>> = OnceCell::new();

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing::subscriber::set_global_default(FmtSubscriber::default())?;

    info!("Setting up database connection");
    let db_url = env::var("SHADOW_DATABASE_URL").unwrap();

    let manager = deadpool_diesel::postgres::Manager::new(db_url, deadpool_diesel::Runtime::Tokio1);
    let pool = deadpool_diesel::postgres::Pool::builder(manager)
        .build()
        .unwrap();

    info!("Connecting to redis");
    let client = redis::Client::open("redis://127.0.0.1:6380?protocol=resp3")?;
    let adapter = RedisAdapterCtr::new_with_redis(&client).await?;

    let (forza_data_sender, forza_data_receiver) = unbounded::<ForzaData>();

    let forza_service = Arc::new(ForzaService::new(pool.clone()));

    let app_state = AppState {
        pool,
        forza_service: forza_service.clone(),
        forza_data_sender,
    };

    tokio::spawn(async move {
        while let Ok(forza_data) = forza_data_receiver.recv() {
            if let Err(e) = forza_service.create_forza_data(forza_data).await {
                warn!("Error creating forza data: {:?}", e);
            }
        }
    });

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
        let _ = io.ns(namespace, socket_on_connect).await?;
        info!("Namespace added: {}", namespace);
    }

    info!("Setting up routes");
    let app = axum::Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .layer(ServiceBuilder::new().layer(layer))
        .with_state(app_state);

    info!("Starting server");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3002").await.unwrap();

    info!("Server started on port 3002");
    axum::serve(listener, app).await?;

    Ok(())
}
