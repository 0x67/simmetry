use crate::app_state::AppState;
use crate::commands::lib::{UdpErrorResponse, UdpSuccessResponse};
use crate::ws_client::create_or_get_ws_client;
use futures_util::FutureExt;
use rmp_serde;
use rs_shared::{constants::GameType, packets::forza::parse_forza_packet, WebsocketPayload};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use std::{
    net::IpAddr,
    str::FromStr,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};
use tauri::{async_runtime::spawn, State};
use tokio::{
    net::UdpSocket,
    sync::{mpsc, Mutex},
};
use uuid::{timestamp, ContextV7, Uuid};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUdpListenerPayload {
    pub game_type: GameType,
    pub port: u16,
    /// Format: "host:port"
    pub forward_ports: Option<Vec<String>>,
}
impl CreateUdpListenerPayload {
    pub fn validate(&self) -> Result<(), String> {
        if let Some(ports) = &self.forward_ports {
            for entry in ports {
                if let Some((ip, port)) = entry.rsplit_once(':') {
                    if IpAddr::from_str(ip).is_err() {
                        return Err(format!("Invalid IP address: {}", ip));
                    }
                    if port.parse::<u16>().is_err() {
                        return Err(format!("Invalid port number: {}", port));
                    }
                } else {
                    return Err(format!("Invalid format (should be IP:port): {}", entry));
                }
            }
        }
        Ok(())
    }
}

#[tauri::command(rename_all = "snake_case")]
pub async fn create_udp_listener(
    payload: CreateUdpListenerPayload,
    state: State<'_, Mutex<AppState>>,
) -> Result<UdpSuccessResponse, UdpErrorResponse> {
    if let Err(e) = payload.validate() {
        return Err(UdpErrorResponse {
            message: e,
            success: false,
        });
    }

    let addr = format!("0.0.0.0:{}", payload.port);
    let socket = UdpSocket::bind(&addr).await.map_err(|e| UdpErrorResponse {
        message: format!("Failed to bind UDP socket: {}", e),
        success: false,
    })?;

    let socket = Arc::new(socket);
    let shutdown_flag = Arc::new(AtomicBool::new(false));
    let ws_client = create_or_get_ws_client(payload.game_type, state.clone()).await;
    let mut state = state.lock().await;

    if state.udp_listeners.contains_key(&payload.port) {
        return Err(UdpErrorResponse {
            message: format!("Port {} is already in use", payload.port),
            success: false,
        });
    }

    state
        .udp_listeners
        .insert(payload.port, (socket.clone(), shutdown_flag.clone()));

    info!(
        "Creating UDP listener on port {} for {}",
        payload.port, payload.game_type
    );

    let (tx, mut rx) = mpsc::channel::<(Vec<u8>, std::net::SocketAddr)>(100);
    let cloned_socket = Arc::clone(&socket);
    let cloned_shutdown_flag = Arc::clone(&shutdown_flag);

    spawn(async move {
        let mut buf = [0; 2048];
        while !cloned_shutdown_flag.load(Ordering::Relaxed) {
            match cloned_socket.recv_from(&mut buf).await {
                Ok((size, src)) => {
                    // ensure the buffer match the original size
                    let sliced_buf = buf[..size].to_vec();

                    match payload.game_type {
                        GameType::FH4 | GameType::FH5 | GameType::FM7 | GameType::FM8 => {
                            let decoded_packet = parse_forza_packet(&sliced_buf).unwrap();
                            if !decoded_packet.is_race_on {
                                continue;
                            }
                        }
                        _ => {
                            // error!("Unsupported game type: {}", payload.game_type);
                            continue;
                        }
                    }

                    if let Err(_) = tx.send((sliced_buf, src)).await {
                        error!("Failed to queue UDP packet for WebSocket emission");
                    }
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    tokio::task::yield_now().await;
                    continue;
                }
                Err(e) => {
                    error!("Error receiving packet: {}", e);
                    break;
                }
            }
        }
        info!("UDP Listener stopped on port {}", payload.port);
    });

    spawn(async move {
        while let Some((data, _)) = rx.recv().await {
            let ts = timestamp::Timestamp::now(ContextV7::new());

            let (seconds, nanos) = ts.to_unix();
            let ts_nanos: u128 = (seconds as u128) * 1_000_000_000 + (nanos as u128);

            let payload = WebsocketPayload {
                id: Uuid::new_v7(ts),
                game_type: payload.game_type,
                timestamp: ts_nanos,
                data,
            };

            let buf = rmp_serde::to_vec(&payload).unwrap();

            if let Err(e) = ws_client
                .emit_with_ack("message", buf, Duration::from_secs(10), |_, _| {
                    async move {
                        // info!("Packet sent to WebSocket");
                    }
                    .boxed()
                })
                .await
            {
                error!("Failed to emit WebSocket message: {}", e);
            }
        }
    });

    Ok(UdpSuccessResponse {
        message: format!("UDP listener created on {}", addr),
        success: true,
    })
}
