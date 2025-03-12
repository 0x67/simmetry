use crate::constants::GameType;
use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct UdpSuccessResponse {
    pub message: String,
    pub success: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UdpErrorResponse {
    pub message: String,
    pub success: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Encode, Decode)]
struct WebsocketPayload {
    #[bincode(with_serde)]
    id: Uuid,
    game_type: GameType,
    timestamp: u128,
    data: Vec<u8>,
}
