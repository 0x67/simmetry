use crate::app_state::AppState;
use crate::commands::lib::{UdpErrorResponse, UdpSuccessResponse};
use serde::{Deserialize, Serialize};
use std::sync::atomic::Ordering;
use tauri::State;
use tokio::sync::Mutex;

#[derive(Debug, Serialize, Deserialize)]
pub struct StopUdpListenerPayload {
    pub port: u16,
    pub host: String,
}

#[tauri::command(rename_all = "snake_case")]
pub async fn stop_udp_listener(
    payload: StopUdpListenerPayload,
    state: State<'_, Mutex<AppState>>,
) -> Result<UdpSuccessResponse, UdpErrorResponse> {
    let mut state = state.lock().await;

    if let Some((socket, shutdown_flag)) = state.udp_listeners.remove(&payload.port) {
        shutdown_flag.store(true, Ordering::Relaxed);
        println!("Stopping UDP listener on port {}", payload.port);

        let _ = socket
            .send_to(&[], format!("127.0.0.1:{}", payload.port))
            .await;

        return Ok(UdpSuccessResponse {
            message: format!("UDP Listener stopped on port {}", payload.port),
            success: true,
        });
    }

    Err(UdpErrorResponse {
        message: format!("No active UDP listener found on port {}", payload.port),
        success: false,
    })
}
