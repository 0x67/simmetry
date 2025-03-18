#[cfg(feature = "db")]
use crate::database::schema::forza_data;
use bincode::{Decode, Encode};
use binrw::BinRead;
use chrono::NaiveDateTime;
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

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "db",
    derive(Insertable, Selectable, Queryable, Identifiable)
)]
#[cfg_attr(feature = "db", diesel(table_name = forza_data))]
#[cfg_attr(feature = "db", diesel(check_for_backend(diesel::pg::Pg)))]
pub struct ForzaData {
    pub id: String,
    pub game_type: ForzaType,
    pub date_time: NaiveDateTime,
    pub is_race_on: bool,

    pub car_id: i32,
    pub car_class: ForzaCarClass,
    pub car_performance_index: i32,
    pub drive_type: ForzaDriveType,
    pub num_cylinders: i32,
    pub track_id: Option<i32>,

    pub engine_max_rpm: f32,
    pub engine_idle_rpm: f32,
    pub engine_current_rpm: f32,

    pub acceleration_x: f32,
    pub acceleration_y: f32,
    pub acceleration_z: f32,

    pub velocity_x: f32,
    pub velocity_y: f32,
    pub velocity_z: f32,

    pub angular_velocity_x: f32,
    pub angular_velocity_y: f32,
    pub angular_velocity_z: f32,

    pub yaw: f32,
    pub pitch: f32,
    pub roll: f32,

    pub normalized_suspension_travel_front_left: f32,
    pub normalized_suspension_travel_front_right: f32,
    pub normalized_suspension_travel_rear_left: f32,
    pub normalized_suspension_travel_rear_right: f32,

    pub tire_slip_ratio_front_left: f32,
    pub tire_slip_ratio_front_right: f32,
    pub tire_slip_ratio_rear_left: f32,
    pub tire_slip_ratio_rear_right: f32,

    pub wheel_rotation_speed_front_left: f32,
    pub wheel_rotation_speed_front_right: f32,
    pub wheel_rotation_speed_rear_left: f32,
    pub wheel_rotation_speed_rear_right: f32,

    pub wheel_on_rumble_strip_front_left: bool,
    pub wheel_on_rumble_strip_front_right: bool,
    pub wheel_on_rumble_strip_rear_left: bool,
    pub wheel_on_rumble_strip_rear_right: bool,

    pub wheel_in_puddle_front_left: f32,
    pub wheel_in_puddle_front_right: f32,
    pub wheel_in_puddle_rear_left: f32,
    pub wheel_in_puddle_rear_right: f32,

    pub surface_rumble_front_left: f32,
    pub surface_rumble_front_right: f32,
    pub surface_rumble_rear_left: f32,
    pub surface_rumble_rear_right: f32,

    pub tire_slip_angle_front_left: f32,
    pub tire_slip_angle_front_right: f32,
    pub tire_slip_angle_rear_left: f32,
    pub tire_slip_angle_rear_right: f32,

    pub tire_combined_slip_front_left: f32,
    pub tire_combined_slip_front_right: f32,
    pub tire_combined_slip_rear_left: f32,
    pub tire_combined_slip_rear_right: f32,

    pub suspension_travel_meters_front_left: f32,
    pub suspension_travel_meters_front_right: f32,
    pub suspension_travel_meters_rear_left: f32,
    pub suspension_travel_meters_rear_right: f32,

    pub position_x: Option<f32>,
    pub position_y: Option<f32>,
    pub position_z: Option<f32>,

    pub speed: Option<f32>,
    pub power: Option<f32>,
    pub torque: Option<f32>,

    pub tire_temp_front_left: Option<f32>,
    pub tire_temp_front_right: Option<f32>,
    pub tire_temp_rear_left: Option<f32>,
    pub tire_temp_rear_right: Option<f32>,

    pub boost: Option<f32>,
    pub fuel: Option<f32>,
    pub distance_traveled: Option<f32>,

    pub best_lap: Option<f32>,
    pub last_lap: Option<f32>,
    pub current_lap: Option<f32>,
    pub current_race_time: Option<f32>,
    pub lap_number: Option<i32>,
    pub position: Option<i32>,

    pub acceleration: Option<i32>,
    pub brake: Option<i32>,
    pub clutch: Option<i32>,
    pub handbrake: Option<i32>,
    pub gear: Option<i32>,
    pub steer: Option<i32>,

    pub normalized_driving_lane: Option<i32>,
    pub normalized_ai_brake_difference: Option<i32>,

    pub tire_wear_front_left: Option<f32>,
    pub tire_wear_front_right: Option<f32>,
    pub tire_wear_rear_left: Option<f32>,
    pub tire_wear_rear_right: Option<f32>,
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
