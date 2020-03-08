//! API specification for F1 2019.

use crate::nineteen::event::EventPacket;
use crate::nineteen::header::PacketType;
use crate::nineteen::lap::LapPacket;
use crate::nineteen::motion::MotionPacket;
use crate::nineteen::participants::ParticipantsPacket;
use crate::nineteen::session::SessionPacket;
use crate::nineteen::setup::CarSetupPacket;
use crate::nineteen::status::CarStatusPacket;
use crate::nineteen::telemetry::TelemetryPacket;
use crate::packet::FromBytes;
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

pub enum Flag {
    Invalid = -1,
    None = 0,
    Green = 1,
    Blue = 2,
    Yellow = 3,
    Red = 4,
}

pub enum Packet {
    Event(EventPacket),
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
    fn peek_packet_id(cursor: &mut Cursor<BytesMut>) -> Result<PacketType, Error> {
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

    fn decode(cursor: &mut Cursor<BytesMut>) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let packet_type = Packet::peek_packet_id(cursor)?;

        let packet = match packet_type {
            PacketType::Motion => Packet::Motion(MotionPacket::from_bytes(cursor)?),
            PacketType::Session => Packet::Session(SessionPacket::from_bytes(cursor)?),
            PacketType::Lap => Packet::Lap(LapPacket::from_bytes(cursor)?),
            PacketType::Event => Packet::Event(EventPacket::from_bytes(cursor)?),
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
