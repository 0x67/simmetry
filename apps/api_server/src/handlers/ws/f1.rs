use crate::app_state::AppState;
use rmpv::Value;
use rs_shared::{
    constants::GameType,
    database::models::f1::{F1Telemetry, F1Type},
    packets::f1::parse_f1_packet,
    WebsocketPayload,
};
use socketioxide::{
    adapter::{Adapter, Emitter},
    extract::{AckSender, Data, SocketRef, State},
    SocketIo,
};
use socketioxide_redis::{drivers::redis::RedisDriver, CustomRedisAdapter};
use tokio::sync::mpsc::UnboundedSender;

async fn f1_socket<A: Adapter>(
    socket: SocketRef<A>,
    Data(data): Data<Value>,
    state: State<AppState>,
) {
    let game_type = match socket.ns() {
        "/F12024" => F1Type::F12024,
        "/F12023" => F1Type::F12023,
        "/F12022" => F1Type::F12022,
        _ => {
            error!("Unknown game type for F1 namespace");
            return;
        }
    };

    info!(ns = socket.ns(), ?socket.id, "Socket.IO connected");
    socket.emit("auth", &data).ok();

    socket.on("ping", |socket: SocketRef<A>| {
        socket.emit("pong", "🏓").ok();
    });

    {
        let f1_telemetry_sender = state.f1_telemetry_sender.clone();
        let file_writer_sender = state.file_writer_sender.clone();

        socket.on("message", move |Data::<Value>(data)| async move {
            if let Some(data) = data.as_slice() {
                handle_f1_packet(data, game_type, f1_telemetry_sender, file_writer_sender).await;
            }
        });
    }

    {
        let f1_telemetry_sender = state.f1_telemetry_sender.clone();
        let file_writer_sender = state.file_writer_sender.clone();

        socket.on(
            "message-ack",
            move |Data::<Value>(data), ack: AckSender<A>| async move {
                if let Some(data) = data.as_slice() {
                    handle_f1_packet(data, game_type, f1_telemetry_sender, file_writer_sender)
                        .await;
                }
                ack.send("{\"success\":true}").unwrap();
            },
        );
    }
}

async fn handle_f1_packet(
    data: &[u8],
    game_type: F1Type,
    f1_telemetry_sender: UnboundedSender<F1Telemetry>,
    file_writer_sender: UnboundedSender<Vec<u8>>,
) {
    let parsed_msgpack = rmp_serde::from_slice::<WebsocketPayload>(&data).unwrap();

    file_writer_sender
        .send(parsed_msgpack.data.clone())
        .unwrap();

    if let Ok(f1_packet) = parse_f1_packet(&parsed_msgpack.data) {
        let insert_f1_telemetry = F1Telemetry::from_udp_packet(
            f1_packet,
            game_type,
            "0195afaa-e643-7c28-8c74-0e695ff6284c".to_string(),
        );

        f1_telemetry_sender.send(insert_f1_telemetry).unwrap();
    }
}

pub async fn create_f1_namespace(
    io: SocketIo<CustomRedisAdapter<Emitter, RedisDriver>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let _ = io.ns(format!("/{}", GameType::F12024), f1_socket).await?;
    let _ = io.ns(format!("/{}", GameType::F12023), f1_socket).await?;
    let _ = io.ns(format!("/{}", GameType::F12022), f1_socket).await?;

    Ok(())
}
