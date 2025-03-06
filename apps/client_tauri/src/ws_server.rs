use std::sync::Arc;

use rust_socketio::{
    asynchronous::{Client, ClientBuilder},
    Payload,
};
use serde_json::json;
use tauri::async_runtime::spawn_blocking;
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
            .namespace(game_type.to_string())
            .connect()
            .await
            .expect("Unable to connect to WebSocket"),
    );
    state.ws_clients.insert(game_type.clone(), client.clone());

    return client;
}
