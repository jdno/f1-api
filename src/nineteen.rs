//! API specification for F1 2019.

mod header;
pub use header::{PacketHeader, PacketType};

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

/// Data for all vehicles is provided as an array. References to the data in
/// this array are made in the form of a vehicle index.
pub type VehicleIndex = u8;
