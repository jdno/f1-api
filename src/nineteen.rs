//! API specification for F1 2019.
//!
//! F1 2019 publishes session and telemetry data through a UDP interface. It defines several
//! different packet types, each containing a particular set of data. These packets are published at
//! different intervals depending on how quickly their data changes.
//!
//! The full API specification can be found here:
//! https://forums.codemasters.com/topic/44592-f1-2019-udp-specification/

use crate::from_bytes::FromBytes;
use crate::nineteen::header::PacketType;
use crate::nineteen::lap::LapPacket;
use crate::nineteen::motion::MotionPacket;
use crate::nineteen::participants::ParticipantsPacket;
use crate::nineteen::session::SessionPacket;
use crate::nineteen::setup::CarSetupPacket;
use crate::nineteen::status::CarStatusPacket;
use crate::nineteen::telemetry::TelemetryPacket;
use bytes::{Buf, BytesMut};
use std::convert::TryFrom;
use std::io::{Cursor, Error, ErrorKind};

mod header;
pub use header::PacketHeader;

pub mod event;
pub mod lap;
pub mod motion;
pub mod participants;
pub mod session;
pub mod setup;
pub mod status;
pub mod telemetry;

/// Flags shown in F1 2019.
///
/// Flags are an essential tool to communicate the status of a race to the drivers on track. A green
/// flag signals the race start or restart, while a yellow flag warns of hazards on track. The red
/// flag aborts a race or session. The blue flag signals that a faster car is approaching from
/// behind.
pub enum Flag {
    Invalid = -1,
    None = 0,
    Green = 1,
    Blue = 2,
    Yellow = 3,
    Red = 4,
}

/// A packet published by F1 2019.
///
/// F1 2019 publishes different packets with different data at different intervals. Each packet is
/// decoded to match an internal representation.
pub enum Packet {
    Lap(LapPacket),
    Motion(MotionPacket),
    Participants(ParticipantsPacket),
    Session(SessionPacket),
    Setup(CarSetupPacket),
    Telemetry(TelemetryPacket),
    Status(CarStatusPacket),
}

/// Data for all vehicles is provided as an array. References to the data in
/// this array are made in the form of a vehicle index.
pub type VehicleIndex = u8;

impl TryFrom<i8> for Flag {
    type Error = Error;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
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
}

impl Packet {
    /// Peek into the packet to determine the packet type.
    ///
    /// The packet contains an id in its header that identifies the type of the packet. To be able
    /// to properly parse the packet, its type must be known beforehand. Using this function, the
    /// packet id can be retrieved from the packet without modifying the cursor.
    fn peek_packet_id(cursor: &mut Cursor<&mut BytesMut>) -> Result<PacketType, Error> {
        cursor.set_position(5 as u64);

        let packet_type = PacketType::try_from(cursor.get_u8())?;

        cursor.set_position(0);
        Ok(packet_type)
    }
}

impl FromBytes for Packet {
    fn buffer_size() -> usize {
        6
    }

    fn decode(cursor: &mut Cursor<&mut BytesMut>) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let packet_type = Packet::peek_packet_id(cursor)?;

        let packet = match packet_type {
            PacketType::Motion => Packet::Motion(MotionPacket::from_bytes(cursor)?),
            PacketType::Session => Packet::Session(SessionPacket::from_bytes(cursor)?),
            PacketType::Lap => Packet::Lap(LapPacket::from_bytes(cursor)?),
            PacketType::Participants => {
                Packet::Participants(ParticipantsPacket::from_bytes(cursor)?)
            }
            PacketType::Setup => Packet::Setup(CarSetupPacket::from_bytes(cursor)?),
            PacketType::Telemetry => Packet::Telemetry(TelemetryPacket::from_bytes(cursor)?),
            PacketType::Status => Packet::Status(CarStatusPacket::from_bytes(cursor)?),
        };

        Ok(packet)
    }
}
