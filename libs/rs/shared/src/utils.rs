use std::{error::Error, fmt, string::FromUtf8Error};

use serde::Serialize;

#[derive(Debug, PartialEq)]
pub(crate) struct MapBoolError<T>(T);

impl<T: fmt::Display + fmt::Debug> fmt::Display for MapBoolError<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid bool value: {}", self.0)
    }
}

impl<T: fmt::Debug + fmt::Display> Error for MapBoolError<T> {}

pub(crate) fn u8_to_bool(value: u8) -> Result<bool, MapBoolError<u8>> {
    match value {
        0 => Ok(false),
        1 => Ok(true),
        _ => Err(MapBoolError(value)),
    }
}

pub(crate) fn u8_to_usize(value: u8) -> usize {
    value as usize
}

pub(crate) fn i32_to_bool(value: i32) -> Result<bool, MapBoolError<i32>> {
    match value {
        0 => Ok(false),
        1 => Ok(true),
        _ => Err(MapBoolError(value)),
    }
}

pub(crate) fn i32_to_usize(value: i32) -> usize {
    value as usize
}

pub(crate) fn read_name(bytes: [u8; 48]) -> Result<String, FromUtf8Error> {
    let first_nul_index = bytes
        .iter()
        .position(|&byte| byte == b'\0')
        .unwrap_or(bytes.len());

    String::from_utf8(bytes[..first_nul_index].to_vec())
}

pub fn json_to_bytes<T>(payload: T) -> Vec<u8>
where
    T: Serialize,
{
    let mut bytes: Vec<u8> = Vec::new();
    serde_json::to_writer(&mut bytes, &payload).unwrap();
    bytes
}
