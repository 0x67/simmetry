use bincode::{Decode, Encode};
use constants::GameType;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod constants;
pub mod packets;
pub mod utils;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Encode, Decode)]
pub struct WebsocketPayload {
    #[bincode(with_serde)]
    pub id: Uuid,
    pub game_type: GameType,
    pub timestamp: u128,
    #[serde(with = "serde_bytes")]
    pub data: Vec<u8>,
}
