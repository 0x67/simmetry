use binrw::BinRead;
use serde::{Deserialize, Serialize};
use strum_macros::Display;

use crate::utils::u8_to_usize;

#[non_exhaustive]
#[derive(
    BinRead,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Copy,
    Clone,
    Debug,
    Hash,
    Serialize,
    Deserialize,
    Display,
)]
#[br(little, repr(u8))]
pub enum F1PacketId {
    /// motion data for player's car
    Motion,
    /// data about the session – track, time left
    Session,
    /// data about all the lap times of cars in the session
    Laps,
    /// various notable events that happen during a session
    Event,
    /// list of participants in the session, mostly relevant for multiplay
    Participants,
    /// car setups data
    CarSetups,
    /// telemetry data for all cars
    CarTelemetry,
    /// status data for all car
    CarStatus,
    /// confirmation at the end of a race
    FinalClassification,
    /// lobby information in multiplayer
    LobbyInfo,
    /// damage status for all cars
    CarDamage,
    /// lap and tyre data for session
    SessionHistory,
    /// tyre sets data
    TyreSets,
    /// extended motion data
    MotionEx,
    /// time trial data
    TimeTrial,
}

#[non_exhaustive]
#[derive(BinRead, PartialEq, PartialOrd, Clone, Debug, Serialize, Deserialize)]
#[br(little)]
pub struct F1PacketHeader {
    /// packet version e.g. 2024
    #[br(
      assert(
          (2022..=2024).contains(&packet_format),
          "Invalid or unsupported packet format: {}",
          packet_format
      )
  )]
    pub packet_format: u16,
    /// Game year - last two digits e.g. 24
    #[br(if(packet_format >= 2023))]
    pub game_year: u8,
    /// Game major version - "X.00"
    pub game_major_version: u8,
    /// Game minor version - "1.XX"
    pub game_minor_version: u8,
    /// Version of this packet type, all start from 1
    pub packet_version: u8,
    /// Identifier for the packet type
    pub packet_id: F1PacketId,
    /// Unique identifier for the session
    pub session_uid: u64,
    /// Session timestamp
    pub session_time: f32,
    /// Identifier for the frame the data was retrieved on
    pub frame_identifier: u32,
    /// Identifier for the frame the data was retrieved on, don't go back after flashback
    #[br(if(packet_format >= 2023))]
    pub overall_frame_identifier: u32,
    /// Index of player's car in the array
    #[br(map(u8_to_usize))]
    pub player_car_index: usize,
    /// Index of secondary player's car in the array (splitscreen), 255 if no second player
    #[br(map(u8_to_usize))]
    pub secondary_player_car_index: usize,
}
