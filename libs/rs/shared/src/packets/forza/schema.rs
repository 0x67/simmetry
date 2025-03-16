use chrono::{Local, NaiveDateTime, TimeZone};
use diesel::{table, Insertable, Queryable, Selectable};
use serde::Serialize;
use serde_json::Value;
use uuid::Uuid;

use crate::constants::GameType;

table! {
  f1_data (id) {
      id -> Uuid,
      game_type -> Text,
      date_time -> Timestamp,
      is_race_on ->  Bool,

      // engine_max_rpm -> Float,
      // engine_idle_rpm -> Float,
      // engine_current_rpm -> Float,

      // acceleration_x -> Float,
      // acceleration_y -> Float,
      // acceleration_z -> Float,

      // velocity_x -> Float,
      // velocity_y -> Float,
      // velocity_z -> Float,

      // angular_velocity_x -> Float,
      // angular_velocity_y -> Float,
      // angular_velocity_z -> Float,

      // yaw -> Float,
      // pitch -> Float,
      // roll -> Float,

      // normalized_suspension_travel_front_left -> Float,
      // normalized_suspension_travel_front_right -> Float,
      // normalized_suspension_travel_rear_left -> Float,
      // normalized_suspension_travel_rear_right -> Float,

      // tire_slip_ratio_front_left -> Float,
      // tire_slip_ratio_front_right -> Float,
      // tire_slip_ratio_rear_left -> Float,
      // tire_slip_ratio_rear_right -> Float,

      // wheel_rotation_speed_front_left -> Float,
      // wheel_rotation_speed_front_right -> Float,
      // wheel_rotation_speed_rear_left -> Float,
      // wheel_rotation_speed_rear_right -> Float,

      // wheel_on_rumble_strip_front_left -> Bool,
      // wheel_on_rumble_strip_front_right -> Bool,
      // wheel_on_rumble_strip_rear_left -> Bool,
      // wheel_on_rumble_strip_rear_right -> Bool,

      // wheel_in_puddle_front_left -> Float,
      // wheel_in_puddle_front_right -> Float,
      // wheel_in_puddle_rear_left -> Float,
      // wheel_in_puddle_rear_right -> Float,

      // surface_rumble_front_left -> Float,
      // surface_rumble_front_right -> Float,
      // surface_rumble_rear_left -> Float,
      // surface_rumble_rear_right -> Float,

      // tire_slip_angle_front_left -> Float,
      // tire_slip_angle_front_right -> Float,
      // tire_slip_angle_rear_left -> Float,
      // tire_slip_angle_rear_right -> Float,

      // tire_combined_slip_front_left -> Float,
      // tire_combined_slip_front_right -> Float,
      // tire_combined_slip_rear_left -> Float,
      // tire_combined_slip_rear_right -> Float,

      // suspension_travel_meters_front_left -> Float,
      // suspension_travel_meters_front_right -> Float,

      // // TODO: Map car id to car name preferably from another tables
      // car_id -> Int4,
  }
}

// TODO: Separate schema to models
#[derive(Insertable)]
#[diesel(table_name = f1_data)]
pub struct InsertF1Data<'a> {
    pub id: &'a Uuid,
    pub game_type: &'a GameType,
    pub date_time: &'a chrono::NaiveDateTime,
    pub is_race_on: &'a bool,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = f1_data)]
pub struct QueryF1Data {
    pub id: Uuid,
    pub game_type: GameType,
    pub date_time: chrono::NaiveDateTime,
    pub is_race_on: bool,
}
