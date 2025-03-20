#[cfg(feature = "db")]
use crate::database::schema::forza_telemetry;
use crate::{database::models::user::User, packets::forza::packet::ForzaPacket};
use bincode::{Decode, Encode};
use binrw::BinRead;
use chrono::NaiveDateTime;
#[cfg(feature = "db")]
use derive_jsonb::AsJsonb;
#[cfg(feature = "db")]
use diesel::prelude::*;
#[cfg(feature = "db")]
use diesel_derive_enum::DbEnum;
use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, Display, EnumIter};
use uuid::Uuid;

#[derive(
    BinRead,
    Hash,
    Default,
    Debug,
    Clone,
    Copy,
    Deserialize,
    Display,
    Serialize,
    Eq,
    PartialEq,
    Encode,
    Decode,
    EnumIter,
    AsRefStr,
)]
#[br(little, repr(i32))]
#[cfg_attr(feature = "db", derive(DbEnum))]
#[cfg_attr(feature = "db", DbValueStyle = "verbatim")]
pub enum ForzaType {
    #[default]
    FH5,
    FH4,
    FM7,
    FM8,
}

#[non_exhaustive]
#[derive(
    BinRead,
    Hash,
    Debug,
    Clone,
    Copy,
    Deserialize,
    Display,
    Serialize,
    Eq,
    PartialEq,
    Encode,
    Decode,
    EnumIter,
    AsRefStr,
    PartialOrd,
)]
#[cfg_attr(feature = "db", derive(DbEnum))]
#[cfg_attr(feature = "db", DbValueStyle = "verbatim")]
#[br(little, repr(i32))]
pub enum ForzaCarClass {
    D,
    C,
    B,
    A,
    S1,
    S2,
    S3,
    X,
}

#[non_exhaustive]
#[derive(
    BinRead,
    Hash,
    Debug,
    Clone,
    Copy,
    Deserialize,
    Display,
    Serialize,
    Eq,
    PartialEq,
    Encode,
    Decode,
    EnumIter,
    AsRefStr,
    PartialOrd,
)]
#[br(little, repr(i32))]
#[cfg_attr(feature = "db", derive(DbEnum))]
#[cfg_attr(feature = "db", DbValueStyle = "verbatim")]
pub enum ForzaDriveType {
    FWD,
    RWD,
    AWD,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "db", derive(AsJsonb))]
pub struct ForzaOrientationData {
    pub yaw: f32,
    pub pitch: f32,
    pub roll: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "db", derive(AsJsonb))]
pub struct ForzaAccelerationData {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "db", derive(AsJsonb))]
pub struct ForzaVelocityData {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "db", derive(AsJsonb))]
pub struct ForzaAngularVelocityData {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "db", derive(AsJsonb))]
pub struct ForzaSuspensionTravelData {
    pub fl: f32,
    pub fr: f32,
    pub rl: f32,
    pub rr: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "db", derive(AsJsonb))]
pub struct ForzaTireSlipRatioData {
    pub fl: f32,
    pub fr: f32,
    pub rl: f32,
    pub rr: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "db", derive(AsJsonb))]
pub struct ForzaWheelRotationSpeedData {
    pub fl: f32,
    pub fr: f32,
    pub rl: f32,
    pub rr: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "db", derive(AsJsonb))]
pub struct ForzaWheelOnRumbleStripData {
    pub fl: bool,
    pub fr: bool,
    pub rl: bool,
    pub rr: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "db", derive(AsJsonb))]
pub struct ForzaWheelInPuddleData {
    pub fl: f32,
    pub fr: f32,
    pub rl: f32,
    pub rr: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "db", derive(AsJsonb))]
pub struct ForzaSurfaceRumbleData {
    pub fl: f32,
    pub fr: f32,
    pub rl: f32,
    pub rr: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "db", derive(AsJsonb))]
pub struct ForzaTireSlipAngleData {
    pub fl: f32,
    pub fr: f32,
    pub rl: f32,
    pub rr: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "db", derive(AsJsonb))]
pub struct ForzaTireCombinedSlipData {
    pub fl: f32,
    pub fr: f32,
    pub rl: f32,
    pub rr: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "db", derive(AsJsonb))]
pub struct ForzaSuspensionTravelMeterData {
    pub fl: f32,
    pub fr: f32,
    pub rl: f32,
    pub rr: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "db", derive(AsJsonb))]
pub struct ForzaPositionData {
    pub x: Option<f32>,
    pub y: Option<f32>,
    pub z: Option<f32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "db", derive(AsJsonb))]
pub struct ForzaTireTemperatureData {
    pub fl: Option<f32>,
    pub fr: Option<f32>,
    pub rl: Option<f32>,
    pub rr: Option<f32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "db", derive(AsJsonb))]
pub struct ForzaTireWearData {
    pub fl: Option<f32>,
    pub fr: Option<f32>,
    pub rl: Option<f32>,
    pub rr: Option<f32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(
    feature = "db",
    derive(Insertable, Selectable, Queryable, Identifiable, Associations)
)]
#[cfg_attr(feature = "db", diesel(table_name = forza_telemetry))]
#[cfg_attr(feature = "db", diesel(check_for_backend(diesel::pg::Pg)))]
#[cfg_attr(feature = "db", diesel(belongs_to(User)))]
#[cfg_attr(feature = "db", diesel(primary_key(user_id, game_type, date_time)))]
pub struct ForzaTelemetry {
    pub id: String,
    pub user_id: String,
    pub game_type: ForzaType,
    pub date_time: NaiveDateTime,
    pub is_race_on: bool,

    pub drive_type: ForzaDriveType,
    pub car_class: ForzaCarClass,
    pub car_id: i32,
    pub car_performance_index: i32,
    pub num_cylinders: i32,

    pub engine_max_rpm: f32,
    pub engine_idle_rpm: f32,
    pub engine_current_rpm: f32,

    pub track_id: Option<i32>,

    pub speed: Option<f32>,
    pub power: Option<f32>,
    pub torque: Option<f32>,
    pub boost: Option<f32>,
    pub fuel: Option<f32>,
    pub distance_traveled: Option<f32>,

    pub best_lap: Option<f32>,
    pub last_lap: Option<f32>,
    pub current_lap: Option<f32>,
    pub current_race_time: Option<f32>,
    pub lap_number: Option<i32>,
    pub race_position: Option<i32>,

    pub acceleration: Option<i32>,
    pub brake: Option<i32>,
    pub clutch: Option<i32>,
    pub handbrake: Option<i32>,
    pub gear: Option<i32>,
    pub steer: Option<i32>,

    pub normalized_driving_lane: Option<i32>,
    pub normalized_ai_brake_difference: Option<i32>,

    pub accelerations: ForzaAccelerationData,
    pub velocities: ForzaVelocityData,
    pub angular_velocities: ForzaAngularVelocityData,
    pub orientations: ForzaOrientationData,
    pub normalized_suspension_travels: ForzaSuspensionTravelData,
    pub tire_slips_ratios: ForzaTireSlipRatioData,
    pub wheel_rotation_speeds: ForzaWheelRotationSpeedData,
    pub wheel_on_rumble_strips: ForzaWheelOnRumbleStripData,
    pub wheel_in_puddles: ForzaWheelInPuddleData,
    pub surface_rumbles: ForzaSurfaceRumbleData,
    pub tire_slip_angles: ForzaTireSlipAngleData,
    pub tire_combined_slips: ForzaTireCombinedSlipData,
    pub suspension_travel_meters: ForzaSuspensionTravelMeterData,
    pub positions: Option<ForzaPositionData>,
    pub tire_temperatures: Option<ForzaTireTemperatureData>,
    pub tire_wears: Option<ForzaTireWearData>,
}

impl ForzaTelemetry {
    pub fn from_udp_packet(packet: ForzaPacket, game_type: ForzaType, user_id: String) -> Self {
        Self {
            id: Uuid::now_v7().to_string(),
            user_id,
            game_type,
            date_time: chrono::Utc::now().naive_utc(),
            is_race_on: packet.is_race_on,
            car_id: packet.car_id,
            car_class: packet.car_class,
            car_performance_index: packet.car_performance_index,
            drive_type: packet.drive_type,
            num_cylinders: packet.num_cylinders,
            track_id: packet.track_id,
            engine_max_rpm: packet.engine_max_rpm,
            engine_idle_rpm: packet.engine_idle_rpm,
            engine_current_rpm: packet.engine_current_rpm,
            accelerations: ForzaAccelerationData {
                x: packet.acceleration_x,
                y: packet.acceleration_y,
                z: packet.acceleration_z,
            },
            angular_velocities: ForzaAngularVelocityData {
                x: packet.angular_velocity_x,
                y: packet.angular_velocity_y,
                z: packet.angular_velocity_z,
            },
            orientations: ForzaOrientationData {
                yaw: packet.yaw,
                pitch: packet.pitch,
                roll: packet.roll,
            },
            velocities: ForzaVelocityData {
                x: packet.velocity_x,
                y: packet.velocity_y,
                z: packet.velocity_z,
            },
            normalized_suspension_travels: ForzaSuspensionTravelData {
                fl: packet.normalized_suspension_travel_front_left,
                fr: packet.normalized_suspension_travel_front_right,
                rl: packet.normalized_suspension_travel_rear_left,
                rr: packet.normalized_suspension_travel_rear_right,
            },
            tire_slips_ratios: ForzaTireSlipRatioData {
                fl: packet.tire_slip_ratio_front_left,
                fr: packet.tire_slip_ratio_front_right,
                rl: packet.tire_slip_ratio_rear_left,
                rr: packet.tire_slip_ratio_rear_right,
            },
            wheel_rotation_speeds: ForzaWheelRotationSpeedData {
                fl: packet.wheel_rotation_speed_front_left,
                fr: packet.wheel_rotation_speed_front_right,
                rl: packet.wheel_rotation_speed_rear_left,
                rr: packet.wheel_rotation_speed_rear_right,
            },
            wheel_on_rumble_strips: ForzaWheelOnRumbleStripData {
                fl: packet.wheel_on_rumble_strip_front_left,
                fr: packet.wheel_on_rumble_strip_front_right,
                rl: packet.wheel_on_rumble_strip_rear_left,
                rr: packet.wheel_on_rumble_strip_rear_right,
            },
            wheel_in_puddles: ForzaWheelInPuddleData {
                fl: packet.wheel_in_puddle_front_left,
                fr: packet.wheel_in_puddle_front_right,
                rl: packet.wheel_in_puddle_rear_left,
                rr: packet.wheel_in_puddle_rear_right,
            },
            surface_rumbles: ForzaSurfaceRumbleData {
                fl: packet.surface_rumble_front_left,
                fr: packet.surface_rumble_front_right,
                rl: packet.surface_rumble_rear_left,
                rr: packet.surface_rumble_rear_right,
            },
            tire_slip_angles: ForzaTireSlipAngleData {
                fl: packet.tire_slip_angle_front_left,
                fr: packet.tire_slip_angle_front_right,
                rl: packet.tire_slip_angle_rear_left,
                rr: packet.tire_slip_angle_rear_right,
            },
            tire_combined_slips: ForzaTireCombinedSlipData {
                fl: packet.tire_combined_slip_front_left,
                fr: packet.tire_combined_slip_front_right,
                rl: packet.tire_combined_slip_rear_left,
                rr: packet.tire_combined_slip_rear_right,
            },
            suspension_travel_meters: ForzaSuspensionTravelMeterData {
                fl: packet.suspension_travel_meters_front_left,
                fr: packet.suspension_travel_meters_front_right,
                rl: packet.suspension_travel_meters_rear_left,
                rr: packet.suspension_travel_meters_rear_right,
            },
            positions: Some(ForzaPositionData {
                x: packet.position_x,
                y: packet.position_y,
                z: packet.position_z,
            }),
            speed: packet.speed,
            power: packet.power,
            torque: packet.torque,
            tire_temperatures: Some(ForzaTireTemperatureData {
                fl: packet.tire_temp_front_left,
                fr: packet.tire_temp_front_right,
                rl: packet.tire_temp_rear_left,
                rr: packet.tire_temp_rear_right,
            }),
            boost: packet.boost,
            fuel: packet.fuel,
            distance_traveled: packet.distance_traveled,
            best_lap: packet.best_lap,
            last_lap: packet.last_lap,
            current_lap: packet.current_lap,
            current_race_time: packet.current_race_time,
            lap_number: Some(packet.lap_number.unwrap().into()),
            race_position: Some(packet.race_position.unwrap().into()),
            acceleration: Some(packet.acceleration.unwrap().into()),
            brake: Some(packet.brake.unwrap().into()),
            clutch: Some(packet.clutch.unwrap().into()),
            handbrake: Some(packet.handbrake.unwrap().into()),
            gear: Some(packet.gear.unwrap().into()),
            steer: Some(packet.steer.unwrap().into()),
            normalized_driving_lane: Some(packet.normalized_driving_lane.unwrap().into()),
            normalized_ai_brake_difference: Some(
                packet.normalized_ai_brake_difference.unwrap().into(),
            ),
            tire_wears: Some(ForzaTireWearData {
                fl: packet.tire_wear_front_left,
                fr: packet.tire_wear_front_right,
                rl: packet.tire_wear_rear_left,
                rr: packet.tire_wear_rear_right,
            }),
        }
    }
}

#[cfg(feature = "db")]
pub(crate) mod export {
    pub use super::ForzaCarClassMapping as ForzaCarClass;
    pub use super::ForzaDriveTypeMapping as ForzaDriveType;
    pub use super::ForzaTypeMapping as ForzaType;
}
