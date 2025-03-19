use crate::app_state::AppState;
use rmpv::Value;
use rs_shared::{
    constants::GameType,
    database::models::forza::{ForzaTelemetry, ForzaType},
    packets::forza::parse_forza_packet,
    WebsocketPayload,
};
use socketioxide::{
    adapter::{Adapter, Emitter},
    extract::{AckSender, Data, SocketRef, State},
    SocketIo,
};
use socketioxide_redis::{drivers::redis::RedisDriver, CustomRedisAdapter};
use uuid::Uuid;

async fn f1_socket<A: Adapter>(
    socket: SocketRef<A>,
    Data(data): Data<Value>,
    state: State<AppState>,
) {
    info!(ns = socket.ns(), ?socket.id, "Socket.IO connected");
    socket.emit("auth", &data).ok();

    socket.on("ping", |socket: SocketRef<A>| {
        info!("Pong received for {:?} namespace", socket.ns());
        socket.emit("pong", "🏓").ok();
    });

    socket.on("message", |Data::<Value>(data)| async move {
        if let Some(data) = data.as_slice() {
            let parsed_msgpack = rmp_serde::from_slice::<WebsocketPayload>(&data).unwrap();
        }
    });

    socket.on(
        "message-ack",
        |socket: SocketRef<A>, Data::<Value>(data), ack: AckSender<A>| {
            info!(?data, "Received event");

            match socket.ns() {
                "FH5" => {
                    if let Some(data) = data.as_slice() {
                        let parsed_msgpack =
                            rmp_serde::from_slice::<WebsocketPayload>(&data).unwrap();

                        info!("Parsed packet: {:?}", parsed_msgpack);

                        let forza_packet = parse_forza_packet(&parsed_msgpack.data);

                        info!("Forza packet: {:?}", forza_packet);
                    }
                }
                _ => {
                    warn!("Received message from unknown namespace: {:?}", socket.ns());
                }
            }
            ack.send(&data).unwrap();
        },
    );
}

pub async fn create_f1_namespace(
    io: SocketIo<CustomRedisAdapter<Emitter, RedisDriver>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let _ = io.ns(format!("/{}", GameType::F12024), f1_socket).await?;
    let _ = io.ns(format!("/{}", GameType::F12023), f1_socket).await?;
    let _ = io.ns(format!("/{}", GameType::F12022), f1_socket).await?;

    Ok(())
}
