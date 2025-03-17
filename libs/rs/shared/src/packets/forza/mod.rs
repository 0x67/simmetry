pub mod packet;
use binrw::io::Cursor;
use binrw::{BinReaderExt, BinResult};
use packet::ForzaPacketData;

pub fn parse_forza_packet<T: AsRef<[u8]>>(data: T) -> BinResult<ForzaPacketData> {
    let byte_size = data.as_ref().len() as u32; // Get the byte size of the data

    let mut cursor = Cursor::new(data);
    let packet: ForzaPacketData = cursor.read_le_args((byte_size,))?;

    Ok(packet)
}
