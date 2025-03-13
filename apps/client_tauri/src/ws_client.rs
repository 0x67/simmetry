use futures_util::FutureExt;
use rs_shared::constants::GameType;
use rust_socketio::{asynchronous::ClientBuilder, TransportType};
use std::{sync::Arc, time::Duration};
use tauri::State;
use tokio::{sync::Mutex, time::sleep};

use crate::app_state::AppState;

pub async fn create_or_get_ws_client(
    game_type: GameType,
    state: State<'_, Mutex<AppState>>,
) -> Arc<rust_socketio::asynchronous::Client> {
    let mut state = state.lock().await;
    // TODO: use env variable
    let url = "http://localhost:3002/";

    if let Some(client) = state.ws_clients.get(&game_type) {
        return client.clone();
    }

    let client = Arc::new(
        ClientBuilder::new(url)
            .transport_type(TransportType::Websocket)
            .namespace(game_type.to_string())
            .on("error", |err, _| {
                async move { error!("Error: {:#?}", err) }.boxed()
            })
            .on("close", |event, _| {
                async move { error!("Close: {:#?}", event) }.boxed()
            })
            .on("pong", move |_, _| {
                async move { info!("Pong received for {:?} namespace", &game_type) }.boxed()
            })
            .connect()
            .await
            .expect("Unable to connect to WebSocket"),
    );

    // Spawn a background task to send periodic pings
    let client_clone = client.clone();
    tauri::async_runtime::spawn(async move {
        loop {
            sleep(Duration::from_secs(10)).await; // Adjust the interval as needed
            if let Err(e) = client_clone.emit("ping", "{}").await {
                error!("Failed to send ping: {:?}", e);
            }
        }
    });

    state.ws_clients.insert(game_type.clone(), client.clone());

    return client;
}
