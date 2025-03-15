use rs_shared::constants::GameType;
use rust_socketio::asynchronous::Client;
use std::{collections::HashMap, sync::Arc};
use tokio::{net::UdpSocket, sync::Mutex};
use tokio_util::{sync::CancellationToken, task::TaskTracker};

#[derive(Default)]
pub struct AppStateInner {
    pub udp_listeners: HashMap<u16, Arc<UdpSocket>>,
    pub udp_listener_trackers: HashMap<u16, TaskTracker>,
    pub udp_listener_tokens: HashMap<u16, CancellationToken>,

    pub ws_clients: HashMap<GameType, Arc<Client>>,
    pub ws_ping_trackers: HashMap<GameType, TaskTracker>,
    pub ws_ping_tokens: HashMap<GameType, CancellationToken>,

    pub ws_emitter_tokens: HashMap<GameType, CancellationToken>,
    pub ws_emitter_trackers: HashMap<GameType, TaskTracker>,

    pub packet_forwarding_trackers: HashMap<GameType, TaskTracker>,
    pub packet_forwarding_tokens: HashMap<GameType, CancellationToken>,

    pub failed_message_retry_trackers: HashMap<GameType, TaskTracker>,
    pub failed_message_retry_tokens: HashMap<GameType, CancellationToken>,
}

pub type AppState = Mutex<AppStateInner>;
