use crate::app_state::AppState;
use crate::commands::lib::{UdpErrorResponse, UdpSuccessResponse};
use crate::constants::GameType;
use crate::ws_server::create_or_get_ws_client;
use serde::{Deserialize, Serialize};
use std::net::IpAddr;
use std::str::FromStr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tauri::async_runtime::spawn;
use tauri::State;
use tokio::net::UdpSocket;
use tokio::sync::Mutex;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUdpListenerPayload {
    pub game_type: GameType,
    pub port: u16,
    /// Format: `"host:port"`
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

    // TODO: When first bind, returns early but emit event later whether it's bound or not
    let socket = UdpSocket::bind(&addr).await.map_err(|e| UdpErrorResponse {
        message: format!("Failed to bind UDP socket: {}", e),
        success: false,
    })?;

    let socket = Arc::new(socket);
    let shutdown_flag = Arc::new(AtomicBool::new(false));
    let ws_client = create_or_get_ws_client(payload.game_type, state.clone()).await;
    let mut state = state.lock().await;
    {
        if state.udp_listeners.contains_key(&payload.port) {
            return Err(UdpErrorResponse {
                message: format!("Port {} is already in use", payload.port),
                success: false,
            });
        }
        state
            .udp_listeners
            .insert(payload.port, (socket.clone(), shutdown_flag.clone()));
    }

    println!("Creating UDP listener on port {}", payload.port);

    let cloned_socket = Arc::clone(&socket);
    let cloned_shutdown_flag = Arc::clone(&shutdown_flag);

    spawn(async move {
        let mut buf = [0; 2048];
        while !cloned_shutdown_flag.load(Ordering::Relaxed) {
            match cloned_socket.recv_from(&mut buf).await {
                Ok((size, src)) => {
                    let sliced_buf = buf[..size].to_vec();
                    println!("Received {} bytes from {}", size, src);
                    println!("Sliced buffer: {:?}", sliced_buf);
                    if let Err(e) = ws_client.emit("message", sliced_buf).await {
                        println!("Failed to emit WebSocket message: {}", e);
                    };
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    tokio::task::yield_now().await; // Avoid busy looping
                    continue;
                }
                Err(e) => {
                    println!("Error receiving packet: {}", e);
                    break;
                }
            }
        }
        println!("UDP Listener stopped on port {}", payload.port);
    });

    Ok(UdpSuccessResponse {
        message: format!("UDP listener created on {}", addr),
        success: true,
    })
}
