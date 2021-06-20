//! Decoder for header prefixing packets sent by F1 2019

use std::io::{Cursor, Error, ErrorKind};

use bitflags::_core::time::Duration;
use bytes::{Buf, BytesMut};

use crate::packet::ensure_packet_size;
use crate::packet::header::{ApiSpec, GameVersion, Header, PacketType};

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

    let api_spec = decode_api_spec(cursor)?;
    let game_version = decode_game_version(cursor);

    cursor.get_u8(); // Move cursor past packet version

    let packet_type = decode_packet_type(cursor)?;
    let session_uid = cursor.get_u64_le();
    let session_time = Duration::from_secs_f32(cursor.get_f32_le());
    let frame_identifier = cursor.get_u32_le();
    let player_car_index = cursor.get_u8();

    Ok(Header::new(
        api_spec,
        game_version,
        packet_type,
        session_uid,
        session_time,
        frame_identifier,
        player_car_index,
    ))
}

fn decode_api_spec(cursor: &mut Cursor<&mut BytesMut>) -> Result<ApiSpec, Error> {
    let value = cursor.get_u16_le();

    match value {
        2019 => Ok(ApiSpec::Nineteen),
        format => Err(Error::new(
            ErrorKind::InvalidData,
            format!("Unknown API specification {}.", format),
        )),
    }
}

fn decode_game_version(cursor: &mut Cursor<&mut BytesMut>) -> Option<GameVersion> {
    Some(GameVersion::new(cursor.get_u8(), cursor.get_u8()))
}

fn decode_packet_type(cursor: &mut Cursor<&mut BytesMut>) -> Result<PacketType, Error> {
    let value = cursor.get_u8();

    match value {
        0 => Ok(PacketType::Motion),
        1 => Ok(PacketType::Session),
        2 => Ok(PacketType::Lap),
        3 => Ok(PacketType::Event),
        4 => Ok(PacketType::Participants),
        5 => Ok(PacketType::Setup),
        6 => Ok(PacketType::Telemetry),
        7 => Ok(PacketType::Status),
        _ => Err(Error::new(
            ErrorKind::InvalidData,
            "Failed to decode packet id.",
        )),
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use bytes::{BufMut, BytesMut};

    use crate::nineteen::header::{decode_header, HEADER_SIZE};
    use crate::packet::header::{ApiSpec, PacketType};

    #[test]
    fn decode_header_with_error() {
        let mut bytes = BytesMut::with_capacity(0);
        let mut cursor = Cursor::new(&mut bytes);

        let packet = decode_header(&mut cursor);
        assert!(packet.is_err());
    }

    #[test]
    fn decode_header_with_success() {
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
        let header = decode_header(&mut cursor).unwrap();

        assert_eq!(ApiSpec::Nineteen, header.api_spec());
        assert_eq!(1, header.game_version().unwrap().major());
        assert_eq!(2, header.game_version().unwrap().minor());
        assert_eq!(PacketType::Motion, header.packet_type());
        assert_eq!(u64::max_value(), header.session_uid());
        assert_eq!(1, header.session_time().as_secs());
        assert_eq!(u32::max_value(), header.frame_identifier());
        assert_eq!(0, header.player_car_index());
    }
}
