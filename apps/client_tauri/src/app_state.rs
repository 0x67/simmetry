use rs_shared::constants::GameType;
use rust_socketio::asynchronous::Client;
use std::collections::HashMap;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use tokio::net::UdpSocket;

#[derive(Default)]
pub struct AppState {
    pub udp_listeners: HashMap<u16, (Arc<UdpSocket>, Arc<AtomicBool>)>,
    pub ws_clients: HashMap<GameType, Arc<Client>>,
}
