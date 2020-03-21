//! Decoder for header prefixing packets sent by F1 2019

use crate::from_bytes::FromBytes;
use crate::nineteen::VehicleIndex;
use crate::packet::header::{GameVersion, Header};
use bitflags::_core::time::Duration;
use bytes::{Buf, BytesMut};
use std::convert::TryFrom;
use std::io::{Cursor, Error, ErrorKind};

/// Decode the header prefixing packets sent by F1 2019
///
/// Each packet sent by F1 2019 is prefixed with a packet header, which contains technical details
/// required to decode the package properly and information about the session the packet belongs to.
/// The latter is extracted from the header and returned to the caller. The technical details are
/// dropped, since their information is encoded in the type system once the packet has been decoded.
pub fn decode_header(cursor: &mut Cursor<&mut BytesMut>) -> Result<Header, Error> {
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

/// The different types of packets published by F1 2019.
///
/// F1 2019 publishes different packets with different data in different intervals. The packet
/// header at the beginning of each UDP packet identifies the type of packet, which is mapped to the
/// `PacketType` enum.
///
/// TODO Remove enum once all packets have been migrated to unified packet format
#[derive(Debug, PartialEq)]
pub enum PacketType {
    Telemetry = 6,
    Status = 7,
}

/// The packet header at the beginning of each UDP packet.
///
/// Each packet published by F1 2019 starts with the packet header. The header provides information
/// that is required to successfully parse the package. Most importantly, it identifies the version
/// of the game (e.g. 2019 or 2018) and the type of packet. Based on this information, the right
/// decoder is picked to turn the UDP packet into a Rust struct.
///
/// TODO Remove struct once all packets have been migrated to unified packet format
pub struct PacketHeader {
    /// The packet format is the version of the packet. Newer games can fall
    /// back to older packet formats to ensure interoperability with existing
    /// tools. Usually, this is the year of the release, e.g. `2018` or `2019`.
    pub packet_format: u16,

    /// The game is versioned using the format `MAJOR.MINOR`. This field
    /// contains the game's major version.
    pub game_major_version: u8,

    /// The game is versioned using the format `MAJOR.MINOR`. This field
    /// contains the game's minor version.
    pub game_minor_version: u8,

    /// The F1 games send different packets containing different data. Each type
    /// of packet can be versioned as well to allow for API changes. The version
    /// of a packet starts at `1` and increments.
    pub packet_version: u8,

    /// The F1 games send different packets containing different data. Each type
    /// of packet has a unique Id that is used to identify, and the properly
    /// parse, the packet.
    pub packet_type: PacketType,

    /// Each session is identified by a unique identifier.
    pub session_uid: u64,

    /// Each packet contains a timestamp, marking the time the data was
    /// captured.
    pub session_time: f32,

    /// Each packet contains an identifier for the frame the data was retrieved
    /// on.
    pub frame_identifier: u32,

    /// The setups and status of cars are published as arrays. This field
    /// indicates which position in these arrays the player's car has.
    pub player_car_index: VehicleIndex,
}

impl TryFrom<u8> for PacketType {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            6 => Ok(PacketType::Telemetry),
            7 => Ok(PacketType::Status),
            _ => Err(Error::new(
                ErrorKind::InvalidData,
                "Failed to decode packet id.",
            )),
        }
    }
}

impl FromBytes for PacketHeader {
    fn buffer_size() -> usize {
        23
    }

    fn decode(cursor: &mut Cursor<&mut BytesMut>) -> Result<Self, Error> {
        Ok(PacketHeader {
            packet_format: cursor.get_u16_le(),
            game_major_version: cursor.get_u8(),
            game_minor_version: cursor.get_u8(),
            packet_version: cursor.get_u8(),
            packet_type: PacketType::try_from(cursor.get_u8())?,
            session_uid: cursor.get_u64_le(),
            session_time: cursor.get_f32_le(),
            frame_identifier: cursor.get_u32_le(),
            player_car_index: cursor.get_u8(),
        })
    }
}

#[cfg(test)]
mod tests {
    use bytes::{BufMut, BytesMut};
    use std::io::Cursor;

    const HEADER_SIZE: usize = 23;

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
