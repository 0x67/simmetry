use crate::app_state::AppState;
use rmpv::Value;
use rs_shared::{
    constants::GameType,
    database::models::forza::{
        ForzaAccelerationData, ForzaAngularVelocityData, ForzaOrientationData, ForzaPositionData,
        ForzaSurfaceRumbleData, ForzaSuspensionTravelData, ForzaSuspensionTravelMeterData,
        ForzaTelemetry, ForzaTireCombinedSlipData, ForzaTireSlipAngleData, ForzaTireSlipRatioData,
        ForzaTireTemperatureData, ForzaTireWearData, ForzaType, ForzaVelocityData,
        ForzaWheelInPuddleData, ForzaWheelOnRumbleStripData, ForzaWheelRotationSpeedData,
    },
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

async fn forza_socket<A: Adapter>(
    socket: SocketRef<A>,
    Data(data): Data<Value>,
    state: State<AppState>,
) {
    info!(ns = socket.ns(), ?socket.id, "Socket.IO connected");
    socket.emit("auth", &data).ok();

    let forza_telemetry_sender = state.forza_telemetry_sender.clone();

    socket.on("ping", |socket: SocketRef<A>| {
        info!("Pong received for {:?} namespace", socket.ns());
        socket.emit("pong", "🏓").ok();
    });

    socket.on("message", |Data::<Value>(data)| async move {
        if let Some(data) = data.as_slice() {
            let parsed_msgpack = rmp_serde::from_slice::<WebsocketPayload>(&data).unwrap();

            if let Ok(forza_packet) = parse_forza_packet(&parsed_msgpack.data) {
                let insert_forza_telemetry = ForzaTelemetry {
                    id: Uuid::now_v7().to_string(),
                    user_id: "0195afaa-e643-7c28-8c74-0e695ff6284c".to_string(),
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
                    accelerations: ForzaAccelerationData {
                        x: forza_packet.acceleration_x,
                        y: forza_packet.acceleration_y,
                        z: forza_packet.acceleration_z,
                    },
                    angular_velocities: ForzaAngularVelocityData {
                        x: forza_packet.angular_velocity_x,
                        y: forza_packet.angular_velocity_y,
                        z: forza_packet.angular_velocity_z,
                    },
                    orientations: ForzaOrientationData {
                        yaw: forza_packet.yaw,
                        pitch: forza_packet.pitch,
                        roll: forza_packet.roll,
                    },
                    velocities: ForzaVelocityData {
                        x: forza_packet.velocity_x,
                        y: forza_packet.velocity_y,
                        z: forza_packet.velocity_z,
                    },
                    normalized_suspension_travels: ForzaSuspensionTravelData {
                        fl: forza_packet.normalized_suspension_travel_front_left,
                        fr: forza_packet.normalized_suspension_travel_front_right,
                        rl: forza_packet.normalized_suspension_travel_rear_left,
                        rr: forza_packet.normalized_suspension_travel_rear_right,
                    },
                    tire_slips_ratios: ForzaTireSlipRatioData {
                        fl: forza_packet.tire_slip_ratio_front_left,
                        fr: forza_packet.tire_slip_ratio_front_right,
                        rl: forza_packet.tire_slip_ratio_rear_left,
                        rr: forza_packet.tire_slip_ratio_rear_right,
                    },
                    wheel_rotation_speeds: ForzaWheelRotationSpeedData {
                        fl: forza_packet.wheel_rotation_speed_front_left,
                        fr: forza_packet.wheel_rotation_speed_front_right,
                        rl: forza_packet.wheel_rotation_speed_rear_left,
                        rr: forza_packet.wheel_rotation_speed_rear_right,
                    },
                    wheel_on_rumble_strips: ForzaWheelOnRumbleStripData {
                        fl: forza_packet.wheel_on_rumble_strip_front_left,
                        fr: forza_packet.wheel_on_rumble_strip_front_right,
                        rl: forza_packet.wheel_on_rumble_strip_rear_left,
                        rr: forza_packet.wheel_on_rumble_strip_rear_right,
                    },
                    wheel_in_puddles: ForzaWheelInPuddleData {
                        fl: forza_packet.wheel_in_puddle_front_left,
                        fr: forza_packet.wheel_in_puddle_front_right,
                        rl: forza_packet.wheel_in_puddle_rear_left,
                        rr: forza_packet.wheel_in_puddle_rear_right,
                    },
                    surface_rumbles: ForzaSurfaceRumbleData {
                        fl: forza_packet.surface_rumble_front_left,
                        fr: forza_packet.surface_rumble_front_right,
                        rl: forza_packet.surface_rumble_rear_left,
                        rr: forza_packet.surface_rumble_rear_right,
                    },
                    tire_slip_angles: ForzaTireSlipAngleData {
                        fl: forza_packet.tire_slip_angle_front_left,
                        fr: forza_packet.tire_slip_angle_front_right,
                        rl: forza_packet.tire_slip_angle_rear_left,
                        rr: forza_packet.tire_slip_angle_rear_right,
                    },
                    tire_combined_slips: ForzaTireCombinedSlipData {
                        fl: forza_packet.tire_combined_slip_front_left,
                        fr: forza_packet.tire_combined_slip_front_right,
                        rl: forza_packet.tire_combined_slip_rear_left,
                        rr: forza_packet.tire_combined_slip_rear_right,
                    },
                    suspension_travel_meters: ForzaSuspensionTravelMeterData {
                        fl: forza_packet.suspension_travel_meters_front_left,
                        fr: forza_packet.suspension_travel_meters_front_right,
                        rl: forza_packet.suspension_travel_meters_rear_left,
                        rr: forza_packet.suspension_travel_meters_rear_right,
                    },
                    positions: Some(ForzaPositionData {
                        x: forza_packet.position_x,
                        y: forza_packet.position_y,
                        z: forza_packet.position_z,
                    }),
                    speed: forza_packet.speed,
                    power: forza_packet.power,
                    torque: forza_packet.torque,
                    tire_temperatures: Some(ForzaTireTemperatureData {
                        fl: forza_packet.tire_temp_front_left,
                        fr: forza_packet.tire_temp_front_right,
                        rl: forza_packet.tire_temp_rear_left,
                        rr: forza_packet.tire_temp_rear_right,
                    }),
                    boost: forza_packet.boost,
                    fuel: forza_packet.fuel,
                    distance_traveled: forza_packet.distance_traveled,
                    best_lap: forza_packet.best_lap,
                    last_lap: forza_packet.last_lap,
                    current_lap: forza_packet.current_lap,
                    current_race_time: forza_packet.current_race_time,
                    lap_number: Some(forza_packet.lap_number.unwrap().into()),
                    race_position: Some(forza_packet.race_position.unwrap().into()),
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
                    tire_wears: Some(ForzaTireWearData {
                        fl: forza_packet.tire_wear_front_left,
                        fr: forza_packet.tire_wear_front_right,
                        rl: forza_packet.tire_wear_rear_left,
                        rr: forza_packet.tire_wear_rear_right,
                    }),
                };

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
    let _ = io.ns(format!("/{}", GameType::FH4), forza_socket).await?;
    let _ = io.ns(format!("/{}", GameType::FH5), forza_socket).await?;
    let _ = io.ns(format!("/{}", GameType::FM7), forza_socket).await?;
    let _ = io.ns(format!("/{}", GameType::FM8), forza_socket).await?;

    Ok(())
}
