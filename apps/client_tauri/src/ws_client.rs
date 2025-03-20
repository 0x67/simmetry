use futures_util::FutureExt;
use rs_shared::constants::GameType;
use rust_socketio::{
    asynchronous::{Client, ClientBuilder},
    TransportType,
};
use std::{env, sync::Arc, time::Duration};
use tauri::{AppHandle, Manager};
use tokio::time::sleep;
use tokio_util::{sync::CancellationToken, task::TaskTracker};

use crate::app_state::AppState;

pub async fn create_ws_client(
    game_type: GameType,
    app_handle: AppHandle,
) -> Arc<rust_socketio::asynchronous::Client> {
    let url = env::var("API_URL").unwrap();

    let game_type = game_type.clone();

    let token = CancellationToken::new();
    let tracker = TaskTracker::new();

    let client = ClientBuilder::new(url)
        .transport_type(TransportType::Websocket)
        .namespace(game_type.to_string())
        .on("open", move |_, _| {
            let game_type = game_type.clone();
            async move { info!("Event: Connected. Namespace: {}", game_type) }.boxed()
        })
        // NOTE:
        // When disconnecting manually, there always be an error event
        // Because the automatic reconnection is always active
        .on("error", move |err, _| {
            let game_type = game_type.clone();
            async move { error!("Event: Error. Namespace: {}, Error: {:#?}", game_type, err) }
                .boxed()
        })
        .on("close", move |event, _| {
            let game_type = game_type.clone();
            async move {
                error!(
                    "Event: Close. Namespace: {}, Event: {:#?}",
                    game_type, event
                )
            }
            .boxed()
        })
        .on("pong", move |_, _| {
            let game_type = game_type.clone();
            async move {
                info!("Event: Pong received. {}", game_type);
            }
            .boxed()
        })
        .connect()
        .await
        .expect("Unable to connect to WebSocket");

    let state = app_handle.state::<AppState>();

    let mut state = state.lock().await;

    let client = Arc::new(client);

    {
        let token = token.clone();
        // remove the clone from #1 method
        let client = client.clone();
        let app_handle = app_handle.clone();
        tracker.spawn(async move {
          tokio::select! {
              _ = token.cancelled() => {
                   // Remove client, token, and tracker from global state
                   let state = app_handle.state::<AppState>();
                   let mut state = state.lock().await;


                  // Gracefully disconnect the client
                  if let Err(e) = client.disconnect().await {
                      error!("Failed to disconnect WebSocket client for {:?}: {:?}", game_type, e);
                  } else {
                      info!("WebSocket client for {:?} disconnected successfully.", game_type);
                      if state.ws_clients.remove(&game_type).is_some() {
                          info!("Removed WebSocket client for {:?}", game_type);
                      }
                  }


                  if state.ws_ping_tokens.remove(&game_type).is_some() {
                      info!("Removed ping token for {:?}", game_type);
                  }

                  if state.ws_ping_trackers.remove(&game_type).is_some() {
                      info!("Removed ping tracker for {:?}", game_type);
                  }

                  info!("Cleanup complete for {:?}", game_type);
              }
              _ = ping(&client) => {
                  println!("Ping task for {:?} exiting normally.", game_type);
              }
          }
      });
    }

    {
        state.ws_ping_trackers.insert(game_type, tracker);
        state.ws_ping_tokens.insert(game_type, token);
        state.ws_clients.insert(game_type, client.clone());
    }

    client
}

async fn ping(client: &Arc<Client>) {
    sleep(Duration::from_secs(10)).await;

    loop {
        if let Err(err) = client.emit("ping", "").await {
            error!("Error pinging server: {:?}", err);
        }
        sleep(Duration::from_secs(10)).await;
    }
}
