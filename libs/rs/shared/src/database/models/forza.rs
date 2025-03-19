use crate::database::models::user::User;
#[cfg(feature = "db")]
use crate::database::schema::forza_telemetry;
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

#[cfg(feature = "db")]
pub(crate) mod export {
    pub use super::ForzaCarClassMapping as ForzaCarClass;
    pub use super::ForzaDriveTypeMapping as ForzaDriveType;
    pub use super::ForzaTypeMapping as ForzaType;
}
