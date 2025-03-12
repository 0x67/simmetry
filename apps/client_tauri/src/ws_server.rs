use std::sync::Arc;

use futures_util::FutureExt;
use rust_socketio::asynchronous::ClientBuilder;
use rust_socketio::TransportType;
use tauri::State;
use tokio::sync::Mutex;

use crate::app_state::AppState;
use crate::constants::GameType;

pub async fn create_or_get_ws_client(
    game_type: GameType,
    state: State<'_, Mutex<AppState>>,
) -> Arc<rust_socketio::asynchronous::Client> {
    let mut state = state.lock().await;
    let url = "http://localhost:3002/";

    if let Some(client) = state.ws_clients.get(&game_type) {
        return client.clone();
    }

    let client = Arc::new(
        ClientBuilder::new(url)
            .transport_type(TransportType::Websocket)
            .namespace(game_type.to_string())
            .on("error", |err, _| {
                async move { eprintln!("Error: {:#?}", err) }.boxed()
            })
            .on("close", |event, _| {
                async move { eprintln!("Close: {:#?}", event) }.boxed()
            })
            .connect()
            .await
            .expect("Unable to connect to WebSocket"),
    );
    state.ws_clients.insert(game_type.clone(), client.clone());

    return client;
}
