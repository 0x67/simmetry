use crate::app_state::AppState;
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
                                engine_max_rpm: forza_packet.engine_max_rpm,
                                engine_idle_rpm: forza_packet.engine_idle_rpm,
                                engine_current_rpm: forza_packet.engine_current_rpm,
                                acceleration_x: forza_packet.acceleration_x,
                                acceleration_y: forza_packet.acceleration_y,
                                acceleration_z: forza_packet.acceleration_z,
                                velocity_x: forza_packet.velocity_x,
                                velocity_y: forza_packet.velocity_y,
                                velocity_z: forza_packet.velocity_z,
                                angular_velocity_x: forza_packet.angular_velocity_x,
                                angular_velocity_y: forza_packet.angular_velocity_y,
                                angular_velocity_z: forza_packet.angular_velocity_z,
                                yaw: forza_packet.yaw,
                                pitch: forza_packet.pitch,
                                roll: forza_packet.roll,
                                normalized_suspension_travel_front_left: forza_packet
                                    .normalized_suspension_travel_front_left,
                                normalized_suspension_travel_front_right: forza_packet
                                    .normalized_suspension_travel_front_right,
                                normalized_suspension_travel_rear_left: forza_packet
                                    .normalized_suspension_travel_rear_left,
                                normalized_suspension_travel_rear_right: forza_packet
                                    .normalized_suspension_travel_rear_right,
                                tire_slip_ratio_front_left: forza_packet.tire_slip_ratio_front_left,
                                tire_slip_ratio_front_right: forza_packet
                                    .tire_slip_ratio_front_right,
                                tire_slip_ratio_rear_left: forza_packet.tire_slip_ratio_rear_left,
                                tire_slip_ratio_rear_right: forza_packet.tire_slip_ratio_rear_right,
                                wheel_rotation_speed_front_left: forza_packet
                                    .wheel_rotation_speed_front_left,
                                wheel_rotation_speed_front_right: forza_packet
                                    .wheel_rotation_speed_front_right,
                                wheel_rotation_speed_rear_left: forza_packet
                                    .wheel_rotation_speed_rear_left,
                                wheel_rotation_speed_rear_right: forza_packet
                                    .wheel_rotation_speed_rear_right,
                                wheel_on_rumble_strip_front_left: forza_packet
                                    .wheel_on_rumble_strip_front_left,
                                wheel_on_rumble_strip_front_right: forza_packet
                                    .wheel_on_rumble_strip_front_right,
                                wheel_on_rumble_strip_rear_left: forza_packet
                                    .wheel_on_rumble_strip_rear_left,
                                wheel_on_rumble_strip_rear_right: forza_packet
                                    .wheel_on_rumble_strip_rear_right,
                                wheel_in_puddle_front_left: forza_packet.wheel_in_puddle_front_left,
                                wheel_in_puddle_front_right: forza_packet
                                    .wheel_in_puddle_front_right,
                                wheel_in_puddle_rear_left: forza_packet.wheel_in_puddle_rear_left,
                                wheel_in_puddle_rear_right: forza_packet.wheel_in_puddle_rear_right,
                                surface_rumble_front_left: forza_packet.surface_rumble_front_left,
                                surface_rumble_front_right: forza_packet.surface_rumble_front_right,
                                surface_rumble_rear_left: forza_packet.surface_rumble_rear_left,
                                surface_rumble_rear_right: forza_packet.surface_rumble_rear_right,
                                tire_slip_angle_front_left: forza_packet.tire_slip_angle_front_left,
                                tire_slip_angle_front_right: forza_packet
                                    .tire_slip_angle_front_right,
                                tire_slip_angle_rear_left: forza_packet.tire_slip_angle_rear_left,
                                tire_slip_angle_rear_right: forza_packet.tire_slip_angle_rear_right,
                                tire_combined_slip_front_left: forza_packet
                                    .tire_combined_slip_front_left,
                                tire_combined_slip_front_right: forza_packet
                                    .tire_combined_slip_front_right,
                                tire_combined_slip_rear_left: forza_packet
                                    .tire_combined_slip_rear_left,
                                tire_combined_slip_rear_right: forza_packet
                                    .tire_combined_slip_rear_right,
                                suspension_travel_meters_front_left: forza_packet
                                    .suspension_travel_meters_front_left,
                                suspension_travel_meters_front_right: forza_packet
                                    .suspension_travel_meters_front_right,
                                suspension_travel_meters_rear_left: forza_packet
                                    .suspension_travel_meters_rear_left,
                                suspension_travel_meters_rear_right: forza_packet
                                    .suspension_travel_meters_rear_right,
                                position_x: forza_packet.position_x,
                                position_y: forza_packet.position_y,
                                position_z: forza_packet.position_z,
                                speed: forza_packet.speed,
                                power: forza_packet.power,
                                torque: forza_packet.torque,
                                tire_temp_front_left: forza_packet.tire_temp_front_left,
                                tire_temp_front_right: forza_packet.tire_temp_front_right,
                                tire_temp_rear_left: forza_packet.tire_temp_rear_left,
                                tire_temp_rear_right: forza_packet.tire_temp_rear_right,
                                boost: forza_packet.boost,
                                fuel: forza_packet.fuel,
                                distance_traveled: forza_packet.distance_traveled,
                                best_lap: forza_packet.best_lap,
                                last_lap: forza_packet.last_lap,
                                current_lap: forza_packet.current_lap,
                                current_race_time: forza_packet.current_race_time,
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
                                tire_wear_front_left: forza_packet.tire_wear_front_left,
                                tire_wear_front_right: forza_packet.tire_wear_front_right,
                                tire_wear_rear_left: forza_packet.tire_wear_rear_left,
                                tire_wear_rear_right: forza_packet.tire_wear_rear_right,
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
