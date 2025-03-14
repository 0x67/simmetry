use crate::app_state::AppState;
use crate::commands::lib::{UdpErrorResponse, UdpSuccessResponse};
use crate::ws_client::create_or_get_ws_client;
use futures_util::FutureExt;
use rmp_serde;
use rs_shared::{constants::GameType, packets::forza::parse_forza_packet, WebsocketPayload};
use rust_socketio::payload;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::net::ToSocketAddrs;
use std::time::SystemTime;
use std::{
    net::IpAddr,
    str::FromStr,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};
use tauri::{async_runtime::spawn, State};
use tokio::io::Interest;
use tokio::time::{sleep, Duration};
use tokio::{
    net::UdpSocket,
    sync::{mpsc, Mutex},
};
use uuid::{timestamp, ContextV7, Uuid};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct QueuedMessage {
    buf: Vec<u8>,
    retries: u8,
    added: SystemTime,
}

/// Assuming the data is sent at 60Hz, we can store only 10 secs of data

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

const MAX_RETRIES: u8 = 3;
const MAX_AGE_SECS: u64 = 10;
const MAX_QUEUE_SIZE: usize = 10 * 60;

// TODO: implement forwarding
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

    if let Some(hosts) = &payload.forward_hosts {
        for host in hosts {
            if let Err(e) = socket.connect(host).await {
                error!("Failed to connect to host: {}", e);
            }

            info!("Connected to host for forwarding: {}", host);
        }
    }

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

    let (ws_tx, mut ws_rx) = mpsc::channel::<(Vec<u8>, std::net::SocketAddr)>(100);
    let (udp_tx, mut udp_rx) = mpsc::channel::<(Vec<u8>, std::net::SocketAddr)>(100);
    let failed_messages = Arc::new(Mutex::new(VecDeque::<QueuedMessage>::new()));

    let cloned_socket = socket.clone();
    let cloned_shutdown_flag = Arc::clone(&shutdown_flag);
    let failed_messages_clone = Arc::clone(&failed_messages);

    spawn(async move {
        while !cloned_shutdown_flag.load(Ordering::Relaxed) {
            let ready = socket.ready(Interest::READABLE).await.unwrap();

            if ready.is_readable() {
                let mut buf = [0; 2048];

                match socket.recv_from(&mut buf).await {
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
                                warn!("Unsupported game type: {}", payload.game_type);
                                continue;
                            }
                        }

                        if ws_tx.try_send((sliced_buf.clone(), src)).is_err() {
                            warn!("UDP queue is full, dropping oldest packet");
                        }

                        if udp_tx.try_send((sliced_buf.clone(), src)).is_err() {
                            warn!("UDP queue is full, dropping oldest packet");
                        }
                    }
                    // False positive error
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
        }
        info!("UDP Listener stopped on port {}", payload.port);
    });

    let socket_clone2 = cloned_socket.clone();

    // Forwarding to other hosts
    spawn(async move {
        while let Some((data, _)) = udp_rx.recv().await {
            let ready = socket_clone2.ready(Interest::WRITABLE).await.unwrap();

            if ready.is_writable() {
                for host in hosts {
                    if let Err(e) = socket_clone2.send_to(&data, host).await {
                        error!("Failed to send packet to host: {}", e);
                    }
                }
            }
        }
    });

    // WS emit to API
    let ws_client_clone1 = ws_client.clone();
    spawn(async move {
        while let Some((data, _)) = ws_rx.recv().await {
            let ts = timestamp::Timestamp::now(ContextV7::new());

            let (seconds, nanos) = ts.to_unix();
            let ts_nanos: u128 = (seconds as u128) * 1_000_000_000 + (nanos as u128);

            let payload = WebsocketPayload {
                id: Uuid::new_v7(ts),
                game_type: payload.game_type,
                timestamp: ts_nanos,
                data: data.clone(),
            };

            let buf = rmp_serde::to_vec(&payload).unwrap();

            if let Err(e) = ws_client
                .emit_with_ack("message", buf.clone(), Duration::from_secs(10), |_, _| {
                    async move {}.boxed()
                })
                .await
            {
                error!("Failed to emit WebSocket message: {}", e);

                let mut queue = failed_messages_clone.lock().await;
                if queue.len() >= MAX_QUEUE_SIZE {
                    // Drop oldest packet if buffer is full
                    queue.pop_front();
                }

                queue.push_back(QueuedMessage {
                    buf: buf.clone(),
                    retries: 0,
                    added: SystemTime::now(),
                });
            }
        }
    });

    let ws_client_clone2 = ws_client_clone1.clone();
    let failed_messages_retry = failed_messages.clone();
    spawn(async move {
        loop {
            sleep(Duration::from_secs(5)).await;

            let mut queue = failed_messages_retry.lock().await;
            while let Some(mut queued_message) = queue.pop_front() {
                if queued_message.retries >= MAX_RETRIES {
                    warn!("Dropping packet after {} retries", MAX_RETRIES);
                    continue;
                }

                if queued_message.added.elapsed().unwrap() > Duration::from_secs(MAX_AGE_SECS) {
                    warn!("Dropping packet after {} seconds", MAX_AGE_SECS);
                    continue;
                }

                if let Err(e) = ws_client_clone2
                    .emit_with_ack(
                        "message",
                        queued_message.buf.clone(),
                        Duration::from_secs(10),
                        |_, _| async move {}.boxed(),
                    )
                    .await
                {
                    error!("Retry failed: {}, re-buffering message", e);
                    // Increment retry count
                    queued_message.retries += 1;
                    // Add back to the queue for another attempt
                    queue.push_back(queued_message);
                    // Stop retrying if the WebSocket is still down
                    break;
                }

                // Maintain max queue size by dropping the oldest packet
                if queue.len() > MAX_QUEUE_SIZE {
                    queue.pop_front();
                    warn!(
                        "Dropped oldest packet due to queue size limit {}",
                        MAX_QUEUE_SIZE
                    );
                }
            }
        }
    });

    Ok(UdpSuccessResponse {
        message: format!("UDP listener created on {}", addr),
        success: true,
    })
}
