use std::io::SeekFrom;

use binrw::BinRead;
use serde::{Deserialize, Serialize};

use crate::utils::i32_to_bool;

const _FM7_SLED_PACKET_SIZE: u32 = 232;
const FM7_DASH_PACKET_SIZE: u32 = 311;
const FH4_SLED_PACKET_SIZE: u32 = 324;
const FM8_SLED_PACKET_SIZE: u32 = 331;

fn get_offset(packet_size: u32) -> u64 {
    match packet_size {
        FH4_SLED_PACKET_SIZE => 12,
        _ => 0,
    }
}

#[non_exhaustive]
#[derive(
    BinRead, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Debug, Hash, Serialize, Deserialize,
)]
#[br(little, repr(i32))]
pub enum ForzaDriveType {
    FWD = 0,
    RWD = 1,
    AWD = 2,
}

#[non_exhaustive]
#[derive(BinRead, PartialEq, PartialOrd, Clone, Debug, Serialize, Deserialize)]
#[br(little, repr(i32))]
pub enum ForzaCarClass {
    D = 0,
    C = 1,
    B = 2,
    A = 3,
    S1 = 4,
    S2 = 5,
    S3 = 6,
    X = 7,
}

#[non_exhaustive]
#[derive(BinRead, PartialEq, PartialOrd, Clone, Debug, Serialize, Deserialize)]
#[br(little, import(packet_size: u32))]
/// Forza packet
pub struct ForzaPacketData {
    // Start of sled data
    #[br(try_map(i32_to_bool))]
    pub is_race_on: bool,

    pub timestamp_ms: u32,

    // can overflow to 0
    pub engine_max_rpm: f32,
    pub engine_idle_rpm: f32,
    pub engine_current_rpm: f32,

    // In the car's local space; X = right, Y = up, Z = forward
    pub acceleration_x: f32,
    pub acceleration_y: f32,
    pub acceleration_z: f32,

    // In the car's local space; X = right, Y = up, Z = forward
    pub velocity_x: f32,
    pub velocity_y: f32,
    pub velocity_z: f32,

    // In the car's local space; X = pitch, Y = yaw, Z = roll
    pub angular_velocity_x: f32,
    pub angular_velocity_y: f32,
    pub angular_velocity_z: f32,

    pub yaw: f32,
    pub pitch: f32,
    pub roll: f32,

    // Suspension travel normalized: 0.0f = max stretch; 1.0 = max compression
    pub normalized_suspension_travel_front_left: f32,
    pub normalized_suspension_travel_front_right: f32,
    pub normalized_suspension_travel_rear_left: f32,
    pub normalized_suspension_travel_rear_right: f32,

    // Tire normalized slip ratio, = 0 means 100% grip and |ratio| > 1.0 means loss of grip.
    pub tire_slip_ratio_front_left: f32,
    pub tire_slip_ratio_front_right: f32,
    pub tire_slip_ratio_rear_left: f32,
    pub tire_slip_ratio_rear_right: f32,

    // wheel rotation speed in rad/s
    pub wheel_rotation_speed_front_left: f32,
    pub wheel_rotation_speed_front_right: f32,
    pub wheel_rotation_speed_rear_left: f32,
    pub wheel_rotation_speed_rear_right: f32,

    // convert i32 to bool
    #[br(try_map(i32_to_bool))]
    pub wheel_on_rumble_strip_front_left: bool,

    // convert i32 to bool
    #[br(try_map(i32_to_bool))]
    pub wheel_on_rumble_strip_front_right: bool,

    // convert i32 to bool
    #[br(try_map(i32_to_bool))]
    pub wheel_on_rumble_strip_rear_left: bool,

    // convert i32 to bool
    #[br(try_map(i32_to_bool))]
    pub wheel_on_rumble_strip_rear_right: bool,

    // From 0 to 1, where 1 is the deepest puddle
    pub wheel_in_puddle_front_left: f32,
    pub wheel_in_puddle_front_right: f32,
    pub wheel_in_puddle_rear_left: f32,
    pub wheel_in_puddle_rear_right: f32,

    // Non-dimensional surface rumble values passed to controller force feedback
    pub surface_rumble_front_left: f32,
    pub surface_rumble_front_right: f32,
    pub surface_rumble_rear_left: f32,
    pub surface_rumble_rear_right: f32,

    // Tire normalized slip angle, = 0 means 100% grip and |angle| > 1.0 means loss of grip.
    pub tire_slip_angle_front_left: f32,
    pub tire_slip_angle_front_right: f32,
    pub tire_slip_angle_rear_left: f32,
    pub tire_slip_angle_rear_right: f32,

    // Tire normalized combined slip, = 0 means 100% grip and |angle| > 1.0 means loss of grip.
    pub tire_combined_slip_front_left: f32,
    pub tire_combined_slip_front_right: f32,
    pub tire_combined_slip_rear_left: f32,
    pub tire_combined_slip_rear_right: f32,

    // Actual suspension travel in meters
    pub suspension_travel_meters_front_left: f32,
    pub suspension_travel_meters_front_right: f32,
    pub suspension_travel_meters_rear_left: f32,
    pub suspension_travel_meters_rear_right: f32,

    // Unique car id for make / model
    pub car_id: i32,

    // Between 0 (D -- worst cars) and 7 (X class -- best cars) inclusive
    pub car_class: ForzaCarClass,

    // Performance index between 0 and 999
    pub car_performance_index: i32,

    // 0 = FWD, 1 = RWD, 2 = AWD
    /// represent `i32`
    pub drive_type: ForzaDriveType,

    // Number of cylinders in the engine
    pub num_cylinders: i32,

    // Start of dash data
    #[br(if(packet_size >= FM7_DASH_PACKET_SIZE))]
    #[br(seek_before = SeekFrom::Start(get_offset(packet_size)  + 232))]
    pub position_x: f32,

    #[br(if(packet_size >= FM7_DASH_PACKET_SIZE))]
    #[br(seek_before = SeekFrom::Start(get_offset(packet_size) + 236))]
    pub position_y: f32,

    #[br(if(packet_size >= FM7_DASH_PACKET_SIZE))]
    #[br(seek_before = SeekFrom::Start(get_offset(packet_size) + 240))]
    pub position_z: f32,

    #[br(if(packet_size >= FM7_DASH_PACKET_SIZE))]
    #[br(seek_before = SeekFrom::Start(get_offset(packet_size) + 244))]
    pub speed: f32,

    #[br(if(packet_size >= FM7_DASH_PACKET_SIZE))]
    #[br(seek_before = SeekFrom::Start(get_offset(packet_size) + 248))]
    pub power: f32,

    #[br(if(packet_size >= FM7_DASH_PACKET_SIZE))]
    #[br(seek_before = SeekFrom::Start(get_offset(packet_size) + 252))]
    pub torque: f32,

    #[br(if(packet_size >= FM7_DASH_PACKET_SIZE))]
    #[br(seek_before = SeekFrom::Start(get_offset(packet_size) + 256))]
    pub tire_temp_front_left: f32,

    #[br(if(packet_size >= FM7_DASH_PACKET_SIZE))]
    #[br(seek_before = SeekFrom::Start(get_offset(packet_size) + 260))]
    pub tire_temp_front_right: f32,

    #[br(if(packet_size >= FM7_DASH_PACKET_SIZE))]
    #[br(seek_before = SeekFrom::Start(get_offset(packet_size) + 264))]
    pub tire_temp_rear_left: f32,

    #[br(if(packet_size >= FM7_DASH_PACKET_SIZE))]
    #[br(seek_before = SeekFrom::Start(get_offset(packet_size) + 268))]
    pub tire_temp_rear_right: f32,

    #[br(if(packet_size >= FM7_DASH_PACKET_SIZE))]
    #[br(seek_before = SeekFrom::Start(get_offset(packet_size) + 272))]
    pub boost: f32,

    #[br(if(packet_size >= FM7_DASH_PACKET_SIZE))]
    #[br(seek_before = SeekFrom::Start(get_offset(packet_size) + 276))]
    pub fuel: f32,

    #[br(if(packet_size >= FM7_DASH_PACKET_SIZE))]
    #[br(seek_before = SeekFrom::Start(get_offset(packet_size) + 280))]
    pub distance_traveled: f32,

    #[br(if(packet_size >= FM7_DASH_PACKET_SIZE))]
    #[br(seek_before = SeekFrom::Start(get_offset(packet_size) + 284))]
    pub best_lap: f32,

    #[br(if(packet_size >= FM7_DASH_PACKET_SIZE))]
    #[br(seek_before = SeekFrom::Start(get_offset(packet_size) + 288))]
    pub last_lap: f32,

    #[br(if(packet_size >= FM7_DASH_PACKET_SIZE))]
    #[br(seek_before = SeekFrom::Start(get_offset(packet_size) + 292))]
    pub current_lap: f32,

    #[br(if(packet_size >= FM7_DASH_PACKET_SIZE))]
    #[br(seek_before = SeekFrom::Start(get_offset(packet_size) + 296))]
    pub current_race_time: f32,

    #[br(if(packet_size >= FM7_DASH_PACKET_SIZE))]
    #[br(seek_before = SeekFrom::Start(get_offset(packet_size) + 300))]
    pub lap_number: u16,

    #[br(if(packet_size >= FM7_DASH_PACKET_SIZE))]
    #[br(seek_before = SeekFrom::Start(get_offset(packet_size) + 302))]
    pub position: u8,

    #[br(if(packet_size >= FM7_DASH_PACKET_SIZE))]
    #[br(seek_before = SeekFrom::Start(get_offset(packet_size) + 303))]
    pub acceleration: u8,

    #[br(if(packet_size >= FM7_DASH_PACKET_SIZE))]
    #[br(seek_before = SeekFrom::Start(get_offset(packet_size) + 304))]
    pub brake: u8,

    #[br(if(packet_size >= FM7_DASH_PACKET_SIZE))]
    #[br(seek_before = SeekFrom::Start(get_offset(packet_size) + 305))]
    pub clutch: u8,

    #[br(if(packet_size >= FM7_DASH_PACKET_SIZE))]
    #[br(seek_before = SeekFrom::Start(get_offset(packet_size) + 306))]
    pub handbrake: u8,

    #[br(if(packet_size >= FM7_DASH_PACKET_SIZE))]
    #[br(seek_before = SeekFrom::Start(get_offset(packet_size) + 307))]
    pub gear: u8,

    #[br(if(packet_size >= FM7_DASH_PACKET_SIZE))]
    #[br(seek_before = SeekFrom::Start(get_offset(packet_size) + 308))]
    pub steer: i8,

    #[br(if(packet_size >= FM7_DASH_PACKET_SIZE))]
    #[br(seek_before = SeekFrom::Start(get_offset(packet_size) + 309))]
    pub normalized_driving_lane: i8,

    #[br(if(packet_size >= FM7_DASH_PACKET_SIZE))]
    #[br(seek_before = SeekFrom::Start(get_offset(packet_size) + 310))]
    pub normalized_ai_brake_difference: i8,

    #[br(if(packet_size >= FM8_SLED_PACKET_SIZE))]
    #[br(seek_before = SeekFrom::Start(get_offset(packet_size) + 311))]
    pub tire_wear_front_left: f32,

    #[br(if(packet_size >= FM8_SLED_PACKET_SIZE))]
    #[br(seek_before = SeekFrom::Start(get_offset(packet_size) + 315))]
    pub tire_wear_front_right: f32,

    #[br(if(packet_size >= FM8_SLED_PACKET_SIZE))]
    #[br(seek_before = SeekFrom::Start(get_offset(packet_size) + 319))]
    pub tire_wear_rear_left: f32,

    #[br(if(packet_size >= FM8_SLED_PACKET_SIZE))]
    #[br(seek_before = SeekFrom::Start(get_offset(packet_size) + 323))]
    pub tire_wear_rear_right: f32,

    #[br(if(packet_size == FM8_SLED_PACKET_SIZE))]
    #[br(seek_before = SeekFrom::Start(get_offset(packet_size) + 327))]
    pub track_id: i32,
}
