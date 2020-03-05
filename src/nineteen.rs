//! API specification for F1 2019.

mod header;
pub use header::{PacketHeader, PacketType};
use std::convert::TryFrom;
use std::io::{Error, ErrorKind};

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
