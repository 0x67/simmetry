#[cfg(feature = "db")]
use crate::database::schema::forza_data;
use bigdecimal::BigDecimal;
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
    // DbEnum,
)]
#[br(little, repr(i32))]
#[cfg_attr(feature = "db", derive(DbEnum))]
#[cfg_attr(feature = "db", DbValueStyle = "verbatim")]
// #[DbValueStyle = "verbatim"]
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
// #[derive(Debug, Clone, PartialEq, Insertable, Selectable, Queryable, Identifiable)]
// #[diesel(table_name = forza_data)]
// #[diesel(check_for_backend(diesel::pg::Pg))]
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

    pub engine_max_rpm: BigDecimal,
    pub engine_idle_rpm: BigDecimal,
    pub engine_current_rpm: BigDecimal,

    pub acceleration_x: BigDecimal,
    pub acceleration_y: BigDecimal,
    pub acceleration_z: BigDecimal,

    pub velocity_x: BigDecimal,
    pub velocity_y: BigDecimal,
    pub velocity_z: BigDecimal,

    pub angular_velocity_x: BigDecimal,
    pub angular_velocity_y: BigDecimal,
    pub angular_velocity_z: BigDecimal,

    pub yaw: BigDecimal,
    pub pitch: BigDecimal,
    pub roll: BigDecimal,

    pub normalized_suspension_travel_front_left: BigDecimal,
    pub normalized_suspension_travel_front_right: BigDecimal,
    pub normalized_suspension_travel_rear_left: BigDecimal,
    pub normalized_suspension_travel_rear_right: BigDecimal,

    pub tire_slip_ratio_front_left: BigDecimal,
    pub tire_slip_ratio_front_right: BigDecimal,
    pub tire_slip_ratio_rear_left: BigDecimal,
    pub tire_slip_ratio_rear_right: BigDecimal,

    pub wheel_rotation_speed_front_left: BigDecimal,
    pub wheel_rotation_speed_front_right: BigDecimal,
    pub wheel_rotation_speed_rear_left: BigDecimal,
    pub wheel_rotation_speed_rear_right: BigDecimal,

    pub wheel_on_rumble_strip_front_left: bool,
    pub wheel_on_rumble_strip_front_right: bool,
    pub wheel_on_rumble_strip_rear_left: bool,
    pub wheel_on_rumble_strip_rear_right: bool,

    pub wheel_in_puddle_front_left: BigDecimal,
    pub wheel_in_puddle_front_right: BigDecimal,
    pub wheel_in_puddle_rear_left: BigDecimal,
    pub wheel_in_puddle_rear_right: BigDecimal,

    pub surface_rumble_front_left: BigDecimal,
    pub surface_rumble_front_right: BigDecimal,
    pub surface_rumble_rear_left: BigDecimal,
    pub surface_rumble_rear_right: BigDecimal,

    pub tire_slip_angle_front_left: BigDecimal,
    pub tire_slip_angle_front_right: BigDecimal,
    pub tire_slip_angle_rear_left: BigDecimal,
    pub tire_slip_angle_rear_right: BigDecimal,

    pub tire_combined_slip_front_left: BigDecimal,
    pub tire_combined_slip_front_right: BigDecimal,
    pub tire_combined_slip_rear_left: BigDecimal,
    pub tire_combined_slip_rear_right: BigDecimal,

    pub suspension_travel_meters_front_left: BigDecimal,
    pub suspension_travel_meters_front_right: BigDecimal,
    pub suspension_travel_meters_rear_left: BigDecimal,
    pub suspension_travel_meters_rear_right: BigDecimal,

    pub position_x: Option<BigDecimal>,
    pub position_y: Option<BigDecimal>,
    pub position_z: Option<BigDecimal>,

    pub speed: Option<BigDecimal>,
    pub power: Option<BigDecimal>,
    pub torque: Option<BigDecimal>,

    pub tire_temp_front_left: Option<BigDecimal>,
    pub tire_temp_front_right: Option<BigDecimal>,
    pub tire_temp_rear_left: Option<BigDecimal>,
    pub tire_temp_rear_right: Option<BigDecimal>,

    pub boost: Option<BigDecimal>,
    pub fuel: Option<BigDecimal>,
    pub distance_traveled: Option<BigDecimal>,

    pub best_lap: Option<BigDecimal>,
    pub last_lap: Option<BigDecimal>,
    pub current_lap: Option<BigDecimal>,
    pub current_race_time: Option<BigDecimal>,
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

    pub tire_wear_front_left: Option<BigDecimal>,
    pub tire_wear_front_right: Option<BigDecimal>,
    pub tire_wear_rear_left: Option<BigDecimal>,
    pub tire_wear_rear_right: Option<BigDecimal>,
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
