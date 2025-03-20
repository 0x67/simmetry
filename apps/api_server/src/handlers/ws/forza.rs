use crate::app_state::AppState;
use rmpv::Value;
use rs_shared::{
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
use tokio::sync::mpsc::UnboundedSender;

async fn forza_socket<A: Adapter>(
    socket: SocketRef<A>,
    Data(data): Data<Value>,
    state: State<AppState>,
) {
    let game_type = match socket.ns() {
        "/FH5" => ForzaType::FH5,
        "/FH4" => ForzaType::FH4,
        "/FM7" => ForzaType::FM7,
        "/FM8" => ForzaType::FM8,
        _ => {
            error!("Unknown game type for Forza namespace");
            return;
        }
    };

    info!(ns = socket.ns(), ?socket.id, "Socket.IO connected");
    socket.emit("auth", &data).ok();

    socket.on("ping", |socket: SocketRef<A>| {
        info!("Pong received for {:?} namespace", socket.ns());
        socket.emit("pong", "🏓").ok();
    });

    {
        let forza_telemetry_sender = state.forza_telemetry_sender.clone();

        socket.on("message", move |Data::<Value>(data)| async move {
            if let Some(data) = data.as_slice() {
                handle_forza_packet(data, game_type, forza_telemetry_sender).await;
            }
        });
    }

    {
        let forza_telemetry_sender = state.forza_telemetry_sender.clone();

        socket.on(
            "message-ack",
            move |Data::<Value>(data), ack: AckSender<A>| async move {
                if let Some(data) = data.as_slice() {
                    handle_forza_packet(data, game_type, forza_telemetry_sender).await;
                }
                ack.send("{\"success\":true}").unwrap();
            },
        );
    }
}

async fn handle_forza_packet(
    data: &[u8],
    game_type: ForzaType,
    forza_telemetry_sender: UnboundedSender<ForzaTelemetry>,
) {
    let parsed_msgpack = rmp_serde::from_slice::<WebsocketPayload>(&data).unwrap();

    if let Ok(forza_packet) = parse_forza_packet(&parsed_msgpack.data) {
        let insert_forza_telemetry = ForzaTelemetry::from_udp_packet(
            forza_packet,
            game_type,
            "0195afaa-e643-7c28-8c74-0e695ff6284c".to_string(),
        );

        forza_telemetry_sender.send(insert_forza_telemetry).unwrap();
    }
}

pub async fn create_forza_namespace(
    io: SocketIo<CustomRedisAdapter<Emitter, RedisDriver>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let _ = io.ns(format!("/{}", ForzaType::FH4), forza_socket).await?;
    let _ = io.ns(format!("/{}", ForzaType::FH5), forza_socket).await?;
    let _ = io.ns(format!("/{}", ForzaType::FM7), forza_socket).await?;
    let _ = io.ns(format!("/{}", ForzaType::FM8), forza_socket).await?;

    Ok(())
}
