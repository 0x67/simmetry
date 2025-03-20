pub mod headers;

#[cfg(feature = "db")]
use crate::database::schema::f1_telemetry;
use crate::{
    database::models::user::User,
    packets::f1::{
        car_damage::CarDamageData,
        car_setups::CarSetupData,
        car_status::CarStatusData,
        car_telemetry::CarTelemetryData,
        enums::MfdPanelIndex,
        event::{EventCode, EventDetails},
        final_classification::FinalClassificationData,
        laps::LapData,
        lobby::LobbyInfoData,
        motion::CarMotionData,
        packet::{
            F1Packet, F1PacketMotionEx, F1PacketSession, F1PacketSessionHistory, F1PacketTimeTrial,
            F1PacketTyreSets,
        },
        participants::ParticipantsData,
    },
};
use bincode::{Decode, Encode};
#[cfg(feature = "db")]
use derive_jsonb::AsJsonb;
#[cfg(feature = "db")]
use diesel::prelude::*;
#[cfg(feature = "db")]
use diesel_derive_enum::DbEnum;
use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, Display, EnumIter};

use chrono::NaiveDateTime;

use crate::packets::f1::headers::F1PacketId;

#[derive(
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
#[cfg_attr(feature = "db", derive(DbEnum))]
#[cfg_attr(feature = "db", DbValueStyle = "verbatim")]
pub enum F1Type {
    #[default]
    F12024,
    F12023,
    F12022,
}

#[cfg(feature = "db")]
pub(crate) mod export {
    pub use super::F1TypeMapping as F1Type;
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "db", derive(AsJsonb))]
pub struct F1Header {
    pub packet_format: u16,
    pub game_major_version: u8,
    pub game_minor_version: u8,
    pub packet_version: u8,
    pub frame_identifier: u32,
    pub overall_frame_identifier: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(
    feature = "db",
    derive(Insertable, Selectable, Queryable, Identifiable, Associations)
)]
#[cfg_attr(feature = "db", diesel(table_name = f1_telemetry))]
#[cfg_attr(feature = "db", diesel(check_for_backend(diesel::pg::Pg)))]
#[cfg_attr(feature = "db", diesel(belongs_to(User)))]
#[cfg_attr(
    feature = "db",
    diesel(primary_key(session_id, user_id, game_type, date_time))
)]
pub struct F1Telemetry {
    pub session_id: i64,
    pub user_id: String,
    pub game_type: F1Type,
    pub date_time: NaiveDateTime,
    pub packet_type: F1PacketId,
    pub session_time: f32,
    pub player_car_index: i32,
    pub secondary_player_car_index: i32,
    pub num_active_cars: i32,
    pub metadata: F1Header,
    pub time_trial_pb_car_index: Option<i32>,
    pub time_trial_rival_car_index: Option<i32>,
    pub event_code: Option<EventCode>,
    pub event: Option<EventDetails>,
    pub motions: Option<Vec<CarMotionData>>,
    pub motion_extended: Option<F1PacketMotionEx>,
    pub session: Option<F1PacketSession>,
    pub laps: Option<Vec<LapData>>,
    pub participants: Option<Vec<ParticipantsData>>,
    pub next_front_wing_value: Option<f32>,
    pub car_setups: Option<Vec<CarSetupData>>,
    pub mfd_panel_index: Option<MfdPanelIndex>,
    pub mfd_panel_index_secondary_player: Option<MfdPanelIndex>,
    pub suggested_gear: Option<i16>,
    pub car_telemetry: Option<Vec<CarTelemetryData>>,
    pub car_status: Option<Vec<CarStatusData>>,
    pub final_classification: Option<Vec<FinalClassificationData>>,
    pub lobby: Option<Vec<LobbyInfoData>>,
    pub car_damage: Option<Vec<CarDamageData>>,
    pub session_history: Option<F1PacketSessionHistory>,
    pub tyre_sets: Option<F1PacketTyreSets>,
    pub time_trial: Option<F1PacketTimeTrial>,
}

impl F1Telemetry {
    pub fn from_udp_packet(packet: F1Packet, game_type: F1Type, user_id: String) -> Self {
        Self {
            session_id: packet.header.session_uid as i64,
            user_id,
            game_type,
            date_time: chrono::Utc::now().naive_utc(),
            packet_type: packet.header.packet_id,
            session_time: packet.header.session_time,
            player_car_index: packet.header.player_car_index as i32,
            secondary_player_car_index: packet.header.secondary_player_car_index as i32,
            metadata: F1Header {
                packet_format: packet.header.packet_format,
                game_major_version: packet.header.game_major_version,
                game_minor_version: packet.header.game_minor_version,
                packet_version: packet.header.packet_version,
                frame_identifier: packet.header.frame_identifier,
                overall_frame_identifier: Some(packet.header.overall_frame_identifier),
            },
            num_active_cars: todo!(),
            time_trial_pb_car_index: todo!(),
            time_trial_rival_car_index: todo!(),
            event_code: todo!(),
            event: todo!(),
            motions: todo!(),
            motion_extended: todo!(),
            session: todo!(),
            laps: todo!(),
            participants: todo!(),
            next_front_wing_value: todo!(),
            car_setups: todo!(),
            mfd_panel_index: todo!(),
            mfd_panel_index_secondary_player: todo!(),
            suggested_gear: todo!(),
            car_telemetry: todo!(),
            car_status: todo!(),
            final_classification: todo!(),
            lobby: todo!(),
            car_damage: todo!(),
            session_history: todo!(),
            tyre_sets: todo!(),
            time_trial: todo!(),
        }
    }
}
