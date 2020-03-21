//! API specification for F1 2019.
//!
//! F1 2019 publishes session and telemetry data through a UDP interface. It defines several
//! different packet types, each containing a particular set of data. These packets are published at
//! different intervals depending on how quickly their data changes.
//!
//! The full API specification can be found here:
//! https://forums.codemasters.com/topic/44592-f1-2019-udp-specification/

use crate::nineteen::event::decode_event;
use crate::nineteen::header::decode_header;
use crate::nineteen::lap::decode_lap_data;
use crate::nineteen::motion::decode_motion;
use crate::nineteen::participants::decode_participants;
use crate::nineteen::session::decode_session;
use crate::nineteen::setup::decode_setups;
use crate::nineteen::status::decode_statuses;
use crate::nineteen::telemetry::decode_telemetry;
use crate::packet::header::PacketType;
use crate::packet::Packet;
use bytes::BytesMut;
use std::io::{Cursor, Error};

mod header;

pub mod event;
pub mod flag;
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

/// Index referencing a car in the packet payloads
///
/// Data for all vehicles is provided as an array. References to the data in
/// this array are made in the form of a vehicle index.
pub type VehicleIndex = u8;

/// Decode a packet sent by F1 2019
///
/// F1 2019 defines its own API specification that is implemented in the `nineteen` module. For each
/// packet type defined in the API specification, a decoder function exists that maps the packet
/// from F1 2019 to the unified packet format of this crate.
pub fn decode_nineteen(cursor: &mut Cursor<&mut BytesMut>) -> Result<Packet, Error> {
    let header = decode_header(cursor)?;
    cursor.set_position(0);

    let packet = match header.packet_type() {
        PacketType::Event => Packet::Event(decode_event(cursor)?),
        PacketType::Lap => Packet::Lap(decode_lap_data(cursor)?),
        PacketType::Motion => Packet::Motion(decode_motion(cursor)?),
        PacketType::Participants => Packet::Participants(decode_participants(cursor)?),
        PacketType::Session => Packet::Session(decode_session(cursor)?),
        PacketType::Setup => Packet::Setup(decode_setups(cursor)?),
        PacketType::Status => Packet::Status(decode_statuses(cursor)?),
        PacketType::Telemetry => Packet::Telemetry(decode_telemetry(cursor)?),
    };

    Ok(packet)
}
