//! Decoder for flags that can be shown to cars

use crate::types::Flag;
use bytes::{Buf, BytesMut};
use std::io::{Cursor, Error, ErrorKind};

/// Decode a flag that can be shown to cars
pub fn decode_flag(cursor: &mut Cursor<&mut BytesMut>) -> Result<Flag, Error> {
    let value = cursor.get_i8();

    match value {
        -1 => Ok(Flag::Invalid),
        0 => Ok(Flag::None),
        1 => Ok(Flag::Green),
        2 => Ok(Flag::Blue),
        3 => Ok(Flag::Yellow),
        4 => Ok(Flag::Red),
        _ => Err(Error::new(ErrorKind::InvalidData, "Failed to decode flag.")),
    }
}
