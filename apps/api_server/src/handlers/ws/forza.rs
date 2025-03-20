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

async fn forza_socket<A: Adapter>(
    socket: SocketRef<A>,
    Data(data): Data<Value>,
    state: State<AppState>,
) {
    let game_type = match socket.ns() {
        "FH5" => ForzaType::FH5,
        "FH4" => ForzaType::FH4,
        "FM7" => ForzaType::FM7,
        "FM8" => ForzaType::FM8,
        _ => panic!("Unknown game type"),
    };

    info!(ns = socket.ns(), ?socket.id, "Socket.IO connected");
    socket.emit("auth", &data).ok();

    let forza_telemetry_sender = state.forza_telemetry_sender.clone();

    socket.on("ping", |socket: SocketRef<A>| {
        info!("Pong received for {:?} namespace", socket.ns());
        socket.emit("pong", "🏓").ok();
    });

    socket.on("message", move |Data::<Value>(data)| async move {
        if let Some(data) = data.as_slice() {
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

pub async fn create_forza_namespace(
    io: SocketIo<CustomRedisAdapter<Emitter, RedisDriver>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let _ = io.ns(format!("/{}", ForzaType::FH4), forza_socket).await?;
    let _ = io.ns(format!("/{}", ForzaType::FH5), forza_socket).await?;
    let _ = io.ns(format!("/{}", ForzaType::FM7), forza_socket).await?;
    let _ = io.ns(format!("/{}", ForzaType::FM8), forza_socket).await?;

    Ok(())
}
