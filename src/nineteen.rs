//! API specification for F1 2019.
//!
//! F1 2019 publishes session and telemetry data through a UDP interface. It defines several
//! different packet types, each containing a particular set of data. These packets are published at
//! different intervals depending on how quickly their data changes.
//!
//! The full API specification can be found here:
//! https://forums.codemasters.com/topic/44592-f1-2019-udp-specification/

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
