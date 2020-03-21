//! Decoder for header prefixing packets sent by F1 2019

use crate::packet::ensure_packet_size;
use crate::packet::header::{GameVersion, Header};
use bitflags::_core::time::Duration;
use bytes::{Buf, BytesMut};
use std::io::{Cursor, Error};

/// Size of the packet header in F1 2019
pub const HEADER_SIZE: usize = 23;

/// Decode the header prefixing packets sent by F1 2019
///
/// Each packet sent by F1 2019 is prefixed with a packet header, which contains technical details
/// required to decode the package properly and information about the session the packet belongs to.
/// The latter is extracted from the header and returned to the caller. The technical details are
/// dropped, since their information is encoded in the type system once the packet has been decoded.
pub fn decode_header(cursor: &mut Cursor<&mut BytesMut>) -> Result<Header, Error> {
    ensure_packet_size(HEADER_SIZE, cursor)?;

    cursor.get_u16_le(); // Move cursor past packet format

    let game_version = Some(GameVersion::new(cursor.get_u8(), cursor.get_u8()));

    cursor.get_u8(); // Move cursor past packet version
    cursor.get_u8(); // Move cursor past packet type

    let session_uid = cursor.get_u64_le();
    let session_time = Duration::from_secs_f32(cursor.get_f32_le());
    let frame_identifier = cursor.get_u32_le();
    let player_car_index = cursor.get_u8();

    Ok(Header::new(
        game_version,
        session_uid,
        session_time,
        frame_identifier,
        player_car_index,
    ))
}

#[cfg(test)]
mod tests {
    use crate::nineteen::header::HEADER_SIZE;
    use bytes::{BufMut, BytesMut};
    use std::io::Cursor;

    #[test]
    fn decode_header() {
        let mut bytes = BytesMut::with_capacity(HEADER_SIZE);
        bytes.put_u16_le(2019);
        bytes.put_u8(1);
        bytes.put_u8(2);
        bytes.put_u8(3);
        bytes.put_u8(0);
        bytes.put_u64_le(u64::max_value());
        bytes.put_f32_le(1.0);
        bytes.put_u32_le(u32::max_value());
        bytes.put_u8(0);

        let mut cursor = Cursor::new(&mut bytes);
        let header = super::decode_header(&mut cursor).unwrap();

        assert_eq!(1, header.game_version().unwrap().major());
        assert_eq!(2, header.game_version().unwrap().minor());
        assert_eq!(u64::max_value(), header.session_uid());
        assert_eq!(1, header.session_time().as_secs());
        assert_eq!(u32::max_value(), header.frame_identifier());
        assert_eq!(0, header.player_car_index());
    }
}
