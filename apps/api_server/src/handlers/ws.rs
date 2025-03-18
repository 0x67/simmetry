use crate::app_state::AppState;
use bigdecimal::{BigDecimal, FromPrimitive};
use rmpv::Value;
use rs_shared::{
    database::models::forza::{ForzaData, ForzaType},
    packets::forza::parse_forza_packet,
    WebsocketPayload,
};
use socketioxide::{
    adapter::Adapter,
    extract::{AckSender, Data, SocketRef, State},
};
use uuid::Uuid;

pub async fn socket_on_connect<A: Adapter>(
    socket: SocketRef<A>,
    Data(data): Data<Value>,
    state: State<AppState>,
) {
    info!(ns = socket.ns(), ?socket.id, "Socket.IO connected");
    socket.emit("auth", &data).ok();

    let forza_data_sender = state.forza_data_sender.clone();

    socket.on("ping", |socket: SocketRef<A>| {
        info!("Pong received for {:?} namespace", socket.ns());
        socket.emit("pong", "🏓").ok();
    });

    socket.on(
        "message",
        |socket: SocketRef<A>, Data::<Value>(data)| async move {
            match socket.ns() {
                "/FH4" | "/FH5" | "/FM7" | "/FM8" => {
                    if let Some(data) = data.as_slice() {
                        let parsed_msgpack =
                            rmp_serde::from_slice::<WebsocketPayload>(&data).unwrap();

                        if let Ok(forza_packet) = parse_forza_packet(&parsed_msgpack.data) {
                            let insert_forza_data = ForzaData {
                                id: Uuid::now_v7().to_string(),
                                game_type: ForzaType::FH5,
                                date_time: chrono::Utc::now().naive_utc(),
                                is_race_on: forza_packet.is_race_on,
                                car_id: forza_packet.car_id,
                                car_class: forza_packet.car_class,
                                car_performance_index: forza_packet.car_performance_index,
                                drive_type: forza_packet.drive_type,
                                num_cylinders: forza_packet.num_cylinders,
                                track_id: forza_packet.track_id,
                                engine_max_rpm: BigDecimal::from_f32(forza_packet.engine_max_rpm)
                                    .unwrap(),
                                engine_idle_rpm: BigDecimal::from_f32(forza_packet.engine_idle_rpm)
                                    .unwrap(),
                                engine_current_rpm: BigDecimal::from_f32(
                                    forza_packet.engine_current_rpm,
                                )
                                .unwrap(),
                                acceleration_x: BigDecimal::from_f32(forza_packet.acceleration_x)
                                    .unwrap(),
                                acceleration_y: BigDecimal::from_f32(forza_packet.acceleration_y)
                                    .unwrap(),
                                acceleration_z: BigDecimal::from_f32(forza_packet.acceleration_z)
                                    .unwrap(),
                                velocity_x: BigDecimal::from_f32(forza_packet.velocity_x).unwrap(),
                                velocity_y: BigDecimal::from_f32(forza_packet.velocity_y).unwrap(),
                                velocity_z: BigDecimal::from_f32(forza_packet.velocity_z).unwrap(),
                                angular_velocity_x: BigDecimal::from_f32(
                                    forza_packet.angular_velocity_x,
                                )
                                .unwrap(),
                                angular_velocity_y: BigDecimal::from_f32(
                                    forza_packet.angular_velocity_y,
                                )
                                .unwrap(),
                                angular_velocity_z: BigDecimal::from_f32(
                                    forza_packet.angular_velocity_z,
                                )
                                .unwrap(),
                                yaw: BigDecimal::from_f32(forza_packet.yaw).unwrap(),
                                pitch: BigDecimal::from_f32(forza_packet.pitch).unwrap(),
                                roll: BigDecimal::from_f32(forza_packet.roll).unwrap(),
                                normalized_suspension_travel_front_left: BigDecimal::from_f32(
                                    forza_packet.normalized_suspension_travel_front_left,
                                )
                                .unwrap(),
                                normalized_suspension_travel_front_right: BigDecimal::from_f32(
                                    forza_packet.normalized_suspension_travel_front_right,
                                )
                                .unwrap(),
                                normalized_suspension_travel_rear_left: BigDecimal::from_f32(
                                    forza_packet.normalized_suspension_travel_rear_left,
                                )
                                .unwrap(),
                                normalized_suspension_travel_rear_right: BigDecimal::from_f32(
                                    forza_packet.normalized_suspension_travel_rear_right,
                                )
                                .unwrap(),
                                tire_slip_ratio_front_left: BigDecimal::from_f32(
                                    forza_packet.tire_slip_ratio_front_left,
                                )
                                .unwrap(),
                                tire_slip_ratio_front_right: BigDecimal::from_f32(
                                    forza_packet.tire_slip_ratio_front_right,
                                )
                                .unwrap(),
                                tire_slip_ratio_rear_left: BigDecimal::from_f32(
                                    forza_packet.tire_slip_ratio_rear_left,
                                )
                                .unwrap(),
                                tire_slip_ratio_rear_right: BigDecimal::from_f32(
                                    forza_packet.tire_slip_ratio_rear_right,
                                )
                                .unwrap(),
                                wheel_rotation_speed_front_left: BigDecimal::from_f32(
                                    forza_packet.wheel_rotation_speed_front_left,
                                )
                                .unwrap(),
                                wheel_rotation_speed_front_right: BigDecimal::from_f32(
                                    forza_packet.wheel_rotation_speed_front_right,
                                )
                                .unwrap(),
                                wheel_rotation_speed_rear_left: BigDecimal::from_f32(
                                    forza_packet.wheel_rotation_speed_rear_left,
                                )
                                .unwrap(),
                                wheel_rotation_speed_rear_right: BigDecimal::from_f32(
                                    forza_packet.wheel_rotation_speed_rear_right,
                                )
                                .unwrap(),
                                wheel_on_rumble_strip_front_left: forza_packet
                                    .wheel_on_rumble_strip_front_left,
                                wheel_on_rumble_strip_front_right: forza_packet
                                    .wheel_on_rumble_strip_front_right,
                                wheel_on_rumble_strip_rear_left: forza_packet
                                    .wheel_on_rumble_strip_rear_left,
                                wheel_on_rumble_strip_rear_right: forza_packet
                                    .wheel_on_rumble_strip_rear_right,
                                wheel_in_puddle_front_left: BigDecimal::from_f32(
                                    forza_packet.wheel_in_puddle_front_left,
                                )
                                .unwrap(),
                                wheel_in_puddle_front_right: BigDecimal::from_f32(
                                    forza_packet.wheel_in_puddle_front_right,
                                )
                                .unwrap(),
                                wheel_in_puddle_rear_left: BigDecimal::from_f32(
                                    forza_packet.wheel_in_puddle_rear_left,
                                )
                                .unwrap(),
                                wheel_in_puddle_rear_right: BigDecimal::from_f32(
                                    forza_packet.wheel_in_puddle_rear_right,
                                )
                                .unwrap(),
                                surface_rumble_front_left: BigDecimal::from_f32(
                                    forza_packet.surface_rumble_front_left,
                                )
                                .unwrap(),
                                surface_rumble_front_right: BigDecimal::from_f32(
                                    forza_packet.surface_rumble_front_right,
                                )
                                .unwrap(),
                                surface_rumble_rear_left: BigDecimal::from_f32(
                                    forza_packet.surface_rumble_rear_left,
                                )
                                .unwrap(),
                                surface_rumble_rear_right: BigDecimal::from_f32(
                                    forza_packet.surface_rumble_rear_right,
                                )
                                .unwrap(),
                                tire_slip_angle_front_left: BigDecimal::from_f32(
                                    forza_packet.tire_slip_angle_front_left,
                                )
                                .unwrap(),
                                tire_slip_angle_front_right: BigDecimal::from_f32(
                                    forza_packet.tire_slip_angle_front_right,
                                )
                                .unwrap(),
                                tire_slip_angle_rear_left: BigDecimal::from_f32(
                                    forza_packet.tire_slip_angle_rear_left,
                                )
                                .unwrap(),
                                tire_slip_angle_rear_right: BigDecimal::from_f32(
                                    forza_packet.tire_slip_angle_rear_right,
                                )
                                .unwrap(),
                                tire_combined_slip_front_left: BigDecimal::from_f32(
                                    forza_packet.tire_combined_slip_front_left,
                                )
                                .unwrap(),
                                tire_combined_slip_front_right: BigDecimal::from_f32(
                                    forza_packet.tire_combined_slip_front_right,
                                )
                                .unwrap(),
                                tire_combined_slip_rear_left: BigDecimal::from_f32(
                                    forza_packet.tire_combined_slip_rear_left,
                                )
                                .unwrap(),
                                tire_combined_slip_rear_right: BigDecimal::from_f32(
                                    forza_packet.tire_combined_slip_rear_right,
                                )
                                .unwrap(),
                                suspension_travel_meters_front_left: BigDecimal::from_f32(
                                    forza_packet.suspension_travel_meters_front_left,
                                )
                                .unwrap(),
                                suspension_travel_meters_front_right: BigDecimal::from_f32(
                                    forza_packet.suspension_travel_meters_front_right,
                                )
                                .unwrap(),
                                suspension_travel_meters_rear_left: BigDecimal::from_f32(
                                    forza_packet.suspension_travel_meters_rear_left,
                                )
                                .unwrap(),

                                suspension_travel_meters_rear_right: BigDecimal::from_f32(
                                    forza_packet.suspension_travel_meters_rear_right,
                                )
                                .unwrap(),
                                position_x: forza_packet
                                    .position_x
                                    .map(BigDecimal::from_f32)
                                    .flatten(),
                                position_y: forza_packet
                                    .position_y
                                    .map(BigDecimal::from_f32)
                                    .flatten(),
                                position_z: forza_packet
                                    .position_z
                                    .map(BigDecimal::from_f32)
                                    .flatten(),
                                speed: forza_packet.speed.map(BigDecimal::from_f32).flatten(),
                                power: forza_packet.power.map(BigDecimal::from_f32).flatten(),
                                torque: forza_packet.torque.map(BigDecimal::from_f32).flatten(),
                                tire_temp_front_left: forza_packet
                                    .tire_temp_front_left
                                    .map(BigDecimal::from_f32)
                                    .flatten(),
                                tire_temp_front_right: forza_packet
                                    .tire_temp_front_right
                                    .map(BigDecimal::from_f32)
                                    .flatten(),
                                tire_temp_rear_left: forza_packet
                                    .tire_temp_rear_left
                                    .map(BigDecimal::from_f32)
                                    .flatten(),
                                tire_temp_rear_right: forza_packet
                                    .tire_temp_rear_right
                                    .map(BigDecimal::from_f32)
                                    .flatten(),
                                boost: forza_packet.boost.map(BigDecimal::from_f32).flatten(),
                                fuel: forza_packet.fuel.map(BigDecimal::from_f32).flatten(),
                                distance_traveled: forza_packet
                                    .distance_traveled
                                    .map(BigDecimal::from_f32)
                                    .flatten(),
                                best_lap: forza_packet.best_lap.map(BigDecimal::from_f32).flatten(),
                                last_lap: forza_packet.last_lap.map(BigDecimal::from_f32).flatten(),
                                current_lap: forza_packet
                                    .current_lap
                                    .map(BigDecimal::from_f32)
                                    .flatten(),
                                current_race_time: forza_packet
                                    .current_race_time
                                    .map(BigDecimal::from_f32)
                                    .flatten(),
                                lap_number: Some(forza_packet.lap_number.unwrap().into()),
                                position: Some(forza_packet.position.unwrap().into()),
                                acceleration: Some(forza_packet.acceleration.unwrap().into()),
                                brake: Some(forza_packet.brake.unwrap().into()),
                                clutch: Some(forza_packet.clutch.unwrap().into()),
                                handbrake: Some(forza_packet.handbrake.unwrap().into()),
                                gear: Some(forza_packet.gear.unwrap().into()),
                                steer: Some(forza_packet.steer.unwrap().into()),
                                normalized_driving_lane: Some(
                                    forza_packet.normalized_driving_lane.unwrap().into(),
                                ),
                                normalized_ai_brake_difference: Some(
                                    forza_packet.normalized_ai_brake_difference.unwrap().into(),
                                ),
                                tire_wear_front_left: forza_packet
                                    .tire_wear_front_left
                                    .map(BigDecimal::from_f32)
                                    .flatten(),
                                tire_wear_front_right: forza_packet
                                    .tire_wear_front_right
                                    .map(BigDecimal::from_f32)
                                    .flatten(),
                                tire_wear_rear_left: forza_packet
                                    .tire_wear_rear_left
                                    .map(BigDecimal::from_f32)
                                    .flatten(),
                                tire_wear_rear_right: forza_packet
                                    .tire_wear_rear_right
                                    .map(BigDecimal::from_f32)
                                    .flatten(),
                            };

                            forza_data_sender.send(insert_forza_data).unwrap();
                        }
                    }
                }
                _ => {
                    warn!("Received message for unknown namespace: {:?}", socket.ns());
                }
            }
            // socket.emit("message-back", &data).ok();
        },
    );

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
