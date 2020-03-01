use crate::nineteen::VehicleIndex;
use crate::packet::FromBytes;
use bytes::{Buf, BytesMut};
use std::convert::TryFrom;
use std::io::{Cursor, Error, ErrorKind};

#[derive(Debug, PartialEq)]
pub enum PacketType {
    Motion = 0,
    Session = 1,
    Lap = 2,
    Event = 3,
    Participants = 4,
    Setup = 5,
    Telemetry = 6,
    Status = 7,
}

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
}

impl FromBytes for PacketHeader {
    fn buffer_size() -> usize {
        23
    }

    fn decode(cursor: &mut Cursor<BytesMut>) -> Result<Self, Error> {
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
    use crate::nineteen::header::PacketHeader;
    use crate::nineteen::PacketType;
    use crate::packet::FromBytes;
    use bytes::{BufMut, BytesMut};
    use std::io::ErrorKind;

    #[test]
    fn from_bytes() {
        let mut bytes = BytesMut::with_capacity(23);
        bytes.put_u16_le(2019);
        bytes.put_u8(1);
        bytes.put_u8(2);
        bytes.put_u8(3);
        bytes.put_u8(0);
        bytes.put_u64_le(u64::max_value());
        bytes.put_f32_le(1.0);
        bytes.put_u32_le(u32::max_value());
        bytes.put_u8(0);

        let packet = PacketHeader::from_bytes(bytes).unwrap();

        assert_eq!(2019, packet.packet_format);
        assert_eq!(1, packet.game_major_version);
        assert_eq!(2, packet.game_minor_version);
        assert_eq!(3, packet.packet_version);
        assert_eq!(PacketType::Motion, packet.packet_type);
        assert_eq!(u64::max_value(), packet.session_uid);
        assert!(packet.session_time - 1.0 < 0.0001);
        assert_eq!(u32::max_value(), packet.frame_identifier);
        assert_eq!(0, packet.player_car_index);
    }

    #[test]
    fn from_bytes_with_invalid_packet_id() {
        let mut bytes = BytesMut::with_capacity(23);
        bytes.put_u16_le(2019);
        bytes.put_u8(1);
        bytes.put_u8(2);
        bytes.put_u8(3);
        bytes.put_u8(8);
        let padding = vec![0u8; 17];
        bytes.put(padding.as_slice());

        match PacketHeader::from_bytes(bytes) {
            Ok(_) => panic!("Expected decoding header with invalid packet id to fail"),
            Err(error) => assert_eq!(ErrorKind::InvalidData, error.kind()),
        }
    }
}
