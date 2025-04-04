use crate::app_state::AppState;
use crate::commands::lib::{UdpErrorResponse, UdpSuccessResponse};
use crate::ws_client::create_ws_client;
use rmp_serde;
use rs_shared::{
    constants::GameType,
    packets::f1::{event::EventCode, headers::F1PacketId, parse_f1_packet},
    packets::forza::parse_forza_packet,
    WebsocketPayload,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    net::{IpAddr, SocketAddr},
    str::FromStr,
    sync::Arc,
    time::SystemTime,
};
use tauri::{
    async_runtime::{channel, spawn, Sender},
    AppHandle, Manager,
};
use tokio::{
    net::UdpSocket,
    sync::mpsc,
    task::JoinSet,
    time::{Duration, Instant},
};
use tokio_util::{sync::CancellationToken, task::TaskTracker};
use uuid::{timestamp, ContextV7, Uuid};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct QueuedMessage {
    buf: Vec<u8>,
    retries: u8,
    added: SystemTime,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateUdpListenerPayload {
    pub game_type: GameType,
    pub port: u16,
    /// Format: "host:port"
    pub forward_hosts: Option<Vec<String>>,
}

impl CreateUdpListenerPayload {
    pub fn validate(&self) -> Result<(), String> {
        if let Some(hosts) = &self.forward_hosts {
            if hosts.len() >= 5 {
                return Err("Too many forward hosts".to_string());
            }
            for host in hosts {
                if let Some((ip, port)) = host.rsplit_once(':') {
                    if IpAddr::from_str(ip).is_err() {
                        return Err(format!("Invalid IP address: {}", ip));
                    }
                    if port.parse::<u16>().is_err() {
                        return Err(format!("Invalid port number: {}", port));
                    }
                } else {
                    return Err(format!("Invalid format (should be IP:port): {}", host));
                }
            }
        }
        Ok(())
    }
}

#[tauri::command(rename_all = "snake_case")]
pub async fn cmd_create_udp_listener(
    app_handle: AppHandle,
    payload: CreateUdpListenerPayload,
) -> Result<UdpSuccessResponse, UdpErrorResponse> {
    if let Err(e) = payload.validate() {
        return Err(UdpErrorResponse {
            message: e,
            success: false,
        });
    }

    let _ = create_ws_client(payload.game_type, app_handle.clone()).await;

    let (ws_tx, ws_rx) = channel::<(Vec<u8>, SocketAddr)>(3600);
    let (udp_tx, udp_rx) = channel::<(Vec<u8>, SocketAddr)>(3600);

    create_udp_listener(
        payload.clone(),
        app_handle.clone(),
        ws_tx.clone(),
        udp_tx.clone(),
    )
    .await;

    if let Some(forward_hosts) = payload.forward_hosts {
        handle_packets_forwarding(
            payload.port,
            payload.game_type,
            forward_hosts,
            app_handle.clone(),
            udp_rx,
        )
        .await;
    }

    handle_ws_emitter(payload.game_type, app_handle.clone(), ws_rx).await;

    Ok(UdpSuccessResponse {
        message: format!("UDP listener created on port {}", payload.port),
        success: true,
    })
}

async fn create_udp_listener(
    payload: CreateUdpListenerPayload,
    app_handle: AppHandle,
    ws_tx: Sender<(Vec<u8>, SocketAddr)>,
    udp_tx: Sender<(Vec<u8>, SocketAddr)>,
) -> Arc<UdpSocket> {
    let addr = format!("0.0.0.0:{}", payload.port);
    let game_type = payload.game_type;
    let socket = UdpSocket::bind(&addr)
        .await
        .map_err(|e| UdpErrorResponse {
            message: format!("Failed to bind UDP socket: {}", e),
            success: false,
        })
        .unwrap();
    info!("UDP socket bound to {}", addr);

    let socket = Arc::new(socket);

    let state = app_handle.state::<AppState>();

    let mut state = state.lock().await;

    let token = CancellationToken::new();
    let tracker = TaskTracker::new();

    {
        let token = token.clone();
        let tracker = tracker.clone();

        state.udp_listener_trackers.insert(payload.port, tracker);
        state.udp_listener_tokens.insert(payload.port, token);
        state.udp_listeners.insert(payload.port, socket.clone());
    }

    {
        let token = token.clone();
        let socket = socket.clone();
        let app_handle = app_handle.clone();

        tracker.spawn(async move {
            let mut buf = [0; 2048];
            let throttle_time = Duration::from_millis(100); // Adjust throttle interval as needed.

            let mut f1_packet_last_sent_times: HashMap<F1PacketId, Instant> = HashMap::new();
            let mut forza_last_sent = Instant::now() - throttle_time;

            loop {
                tokio::select! {
                    _ = token.cancelled() => {
                        let state = app_handle.state::<AppState>();
                        let mut state = state.lock().await;

                        state.udp_listener_trackers.remove(&payload.port);
                        state.udp_listener_tokens.remove(&payload.port);
                        state.udp_listeners.remove(&payload.port);

                        info!("Stop UDP listener. Port: {}, Game Type: {}", payload.port, game_type);

                        break;
                    }
                    result = socket.recv_from(&mut buf) => {
                        match result {
                            Ok((size, src)) => {
                                let sliced_buf = buf[..size].to_vec();

                                match game_type {
                                    GameType::FH4 | GameType::FH5 | GameType::FM7 | GameType::FM8 => {
                                        let packet = match parse_forza_packet(&sliced_buf) {
                                            Ok(packet) => packet,
                                            Err(e) => {
                                                #[cfg(any(debug_assertions))]
                                                error!("Failed to parse Forza packet: {}", e);
                                                continue;
                                            }
                                        };

                                        if !packet.is_race_on {
                                            continue;
                                        }

                                        let now = Instant::now();
                                        // Throttle Forza packets regardless of their content.
                                        if now.duration_since(forza_last_sent) < throttle_time {
                                            continue;
                                        }
                                        forza_last_sent = now;
                                    }
                                    GameType::F12022 | GameType::F12023 | GameType::F12024 => {
                                        let packet = match parse_f1_packet(&sliced_buf) {
                                            Ok(packet) => packet,
                                            Err(e) => {
                                                #[cfg(any(debug_assertions))]
                                                error!("Failed to parse F1 packet: {}", e);
                                                continue;
                                            }
                                        };

                                        if packet.header.session_uid == 0 {
                                            continue;
                                        }

                                        if let Some(ref event) = packet.event {
                                          if event.code == EventCode::BUTN {
                                              continue;
                                          }
                                        }

                                        let now = Instant::now();

                                        if !is_f1_packet_allowed(packet.header.packet_id) {
                                            let last_sent_time = f1_packet_last_sent_times
                                                .get(&packet.header.packet_id)
                                                .copied()
                                                .unwrap_or_else(|| Instant::now() - throttle_time);

                                            if now.duration_since(last_sent_time) < throttle_time {
                                                continue;
                                            }

                                            f1_packet_last_sent_times.insert(packet.header.packet_id, now);
                                        }
                                    }
                                    _ => {
                                        #[cfg(any(debug_assertions))]
                                        warn!("Unsupported game type: {}", game_type);
                                        continue;
                                    }
                                }

                                if ws_tx.try_send((sliced_buf.clone(), src)).is_err() {
                                    #[cfg(any(debug_assertions))]
                                    warn!("WS queue is full, dropping oldest packet");
                                }
                                if udp_tx.try_send((sliced_buf.clone(), src)).is_err() {
                                    #[cfg(any(debug_assertions))]
                                    warn!("UDP queue is full, dropping oldest packet");
                                }
                            }
                            // false positive
                            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                                tokio::task::yield_now().await;
                            }
                            Err(e) => {
                                #[cfg(any(debug_assertions))]
                                error!("Error receiving packet: {}", e);
                                break;
                            }
                        }
                    }
                }
            }
        });
    }

    socket
}

fn is_f1_packet_allowed(packet_id: F1PacketId) -> bool {
    match packet_id {
        F1PacketId::CarDamage
        | F1PacketId::TimeTrial
        | F1PacketId::TyreSets
        | F1PacketId::SessionHistory
        | F1PacketId::FinalClassification
        | F1PacketId::CarSetups
        | F1PacketId::Participants
        | F1PacketId::Event
        | F1PacketId::Session
        | F1PacketId::LobbyInfo => true,
        _ => false,
    }
}

async fn handle_ws_emitter(
    game_type: GameType,
    app_handle: AppHandle,
    mut ws_rx: mpsc::Receiver<(Vec<u8>, SocketAddr)>,
) {
    let state = app_handle.state::<AppState>();
    let mut state = state.lock().await;

    let tracker = TaskTracker::new();
    let token = CancellationToken::new();

    {
        let token = token.clone();
        let tracker = tracker.clone();
        let app_handle = app_handle.clone();
        let ws_client = state.ws_clients.get(&game_type).unwrap().clone();

        state.ws_emitter_trackers.insert(game_type, tracker.clone());
        state.ws_emitter_tokens.insert(game_type, token.clone());

        tracker.spawn(async move {
            loop {
                tokio::select! {
                    _ = token.cancelled() => {
                        let state = app_handle.state::<AppState>();
                        let mut state = state.lock().await;

                        state.ws_emitter_trackers.remove(&game_type);
                        state.ws_emitter_tokens.remove(&game_type);
                        state.ws_clients.remove(&game_type);

                        break;
                    }
                    result = ws_rx.recv() => {
                        if let Some((buf, _)) = result {
                          let ts = timestamp::Timestamp::now(ContextV7::new());
                          let (seconds, nanos) = ts.to_unix();
                          let ts_nanos: u128 = (seconds as u128) * 1_000_000_000 + (nanos as u128);

                          let payload = WebsocketPayload {
                              id: Uuid::new_v7(ts),
                              game_type,
                              timestamp: ts_nanos,
                              data: buf.clone(),
                          };

                          let ws_payload = match rmp_serde::to_vec(&payload) {
                              Ok(vec) => vec,
                              Err(e) => {
                                  #[cfg(any(debug_assertions))]
                                  error!("Failed to serialize websocket payload: {}", e);
                                  continue;
                              }
                          };

                          if let Err(e) = ws_client.emit("message", ws_payload).await {
                              #[cfg(any(debug_assertions))]
                              error!("Failed to emit WebSocket message: {}", e);
                          }
                        }
                    }
                }
            }
        });
    }
}

async fn handle_packets_forwarding(
    port: u16,
    game_type: GameType,
    forward_hosts: Vec<String>,
    app_handle: AppHandle,
    mut udp_rx: mpsc::Receiver<(Vec<u8>, SocketAddr)>,
) {
    let state = app_handle.state::<AppState>();
    let mut state = state.lock().await;

    let tracker = TaskTracker::new();
    let token = CancellationToken::new();

    {
        let token = token.clone();
        let tracker = tracker.clone();
        let socket = state.udp_listeners.get(&port).unwrap().clone();
        let app_handle = app_handle.clone();

        state
            .packet_forwarding_trackers
            .insert(game_type, tracker.clone());
        state
            .packet_forwarding_tokens
            .insert(game_type, token.clone());

        tracker.spawn(async move {
            loop {
                tokio::select! {
                    _ = token.cancelled() => {
                        let state = app_handle.state::<AppState>();
                        let mut state = state.lock().await;

                        state.packet_forwarding_trackers.remove(&game_type);
                        state.packet_forwarding_tokens.remove(&game_type);

                        break;
                    }
                    result = udp_rx.recv() => {
                        if let Some((buf, _)) = result {
                          let tasks: JoinSet<_> = forward_hosts.iter().map(|host| {
                          let socket = socket.clone();
                          let buf = buf.clone();
                          let host = host.clone();

                          spawn(async move {
                                if let Err(e) = socket.send_to(&buf, &host).await {
                                    #[cfg(any(debug_assertions))]
                                    error!("Failed to send packet to {}: {}", host, e);
                                }
                            })
                          })
                          .collect();

                          tasks.join_all().await;
                        }
                    }
                }
            }
        });
    }
}
