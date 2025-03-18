use binrw::{BinReaderExt, BinResult};
use packet::F1Packet;
use std::io::Cursor;

pub mod car_damage;
pub mod car_setups;
pub mod car_status;
pub mod car_telemetry;
pub mod constants;
pub mod enums;
pub mod event;
pub mod final_classification;
pub mod headers;
pub mod laps;
pub mod lobby;
pub mod motion;
pub mod packet;
pub mod participants;
pub mod session;
pub mod session_history;
pub mod time_trial;
pub mod tyre_sets;

pub fn parse_f1_packet<T: AsRef<[u8]>>(data: T) -> BinResult<F1Packet> {
    let mut cursor = Cursor::new(data);
    let packet: F1Packet = cursor.read_le()?;

    Ok(packet)
}
