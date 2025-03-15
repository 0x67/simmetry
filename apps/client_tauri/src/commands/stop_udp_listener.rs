use crate::app_state::AppState;
use crate::commands::lib::{UdpErrorResponse, UdpSuccessResponse};
use rs_shared::constants::GameType;
use serde::{Deserialize, Serialize};
use tauri::State;

#[derive(Debug, Serialize, Deserialize)]
pub struct StopUdpListenerPayload {
    pub port: u16,
    pub host: String,
    pub game_type: GameType,
}

#[tauri::command(rename_all = "snake_case")]
pub async fn cmd_stop_udp_listener(
    payload: StopUdpListenerPayload,
    state: State<'_, AppState>,
) -> Result<UdpSuccessResponse, UdpErrorResponse> {
    let state = state.lock().await;

    if let Some(tracker) = state.ws_ping_trackers.get(&payload.game_type) {
        let token = state.ws_ping_tokens.get(&payload.game_type).unwrap();
        tracker.close();
        token.cancel();
    }

    if let Some(_) = state.udp_listener_trackers.get(&payload.port) {
        let token = state.udp_listener_tokens.get(&payload.port).unwrap();
        let tracker = state.udp_listener_trackers.get(&payload.port).unwrap();
        tracker.close();
        token.cancel();
    }

    if let Some(_) = state.packet_forwarding_trackers.get(&payload.game_type) {
        let token = state
            .packet_forwarding_tokens
            .get(&payload.game_type)
            .unwrap();
        let tracker = state
            .packet_forwarding_trackers
            .get(&payload.game_type)
            .unwrap();
        tracker.close();
        token.cancel();
    }

    if let Some(_) = state.ws_emitter_trackers.get(&payload.game_type) {
        let token = state.ws_emitter_tokens.get(&payload.game_type).unwrap();
        let tracker = state.ws_emitter_trackers.get(&payload.game_type).unwrap();
        tracker.close();
        token.cancel();
    }

    Ok(UdpSuccessResponse {
        message: format!("UDP Listener stopped on port {}", payload.port),
        success: true,
    })
}
