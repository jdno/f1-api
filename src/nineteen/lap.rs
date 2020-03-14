//! Packet with lap data for all 20 drivers in the session

use crate::nineteen::PacketHeader;
use crate::packet::FromBytes;
use bytes::{Buf, BytesMut};
use std::convert::TryFrom;
use std::io::{Cursor, Error, ErrorKind};

/// The status each driver can have.
///
/// Each driver in the session has a status. The status indicates what the driver is currently
/// doing, e.g. if he is in the garage or on track.
pub enum DriverStatus {
    InGarage = 0,
    FlyingLap = 1,
    InLap = 2,
    OutLap = 3,
    OnTrack = 4,
}

/// The status of a driver during a pit stop.
///
/// When a driver is pitting, a separate status is assigned to them. While the driver is on the pit
/// lane, the status is `Pitting`. When the driver is on the pit area, it is `InPits`.
pub enum PitStatus {
    None = 0,
    Pitting = 1,
    InPits = 2,
}

/// The status of a driver's results.
///
/// The results of a driver can have a different status, based on certain criteria of the session.
/// A driver can get disqualified for rule infringements for example, or a driver might retire
/// during a race.
pub enum ResultStatus {
    Invalid = 0,
    Inactive = 1,
    Active = 2,
    Finished = 3,
    Disqualified = 4,
    NotClassified = 5,
    Retired = 6,
}

/// The three sectors of a race track in F1.
pub enum Sector {
    First = 0,
    Second = 1,
    Third = 2,
}

/// Data about a driver and their lap.
///
/// For each driver in the session, a set of lap data is published. It contains data on the current
/// lap, e.g. the current lap time and the sector the driver is currently in, but also the time of
/// the last and best laps, as well as the driver status.
pub struct Lap {
    /// Last lap time in seconds.
    pub last_lap_time: f32,

    /// Current time around the lap in seconds.
    pub current_lap_time: f32,

    /// Best lap time in the session in seconds.
    pub best_lap_time: f32,

    /// Time in sector 1 in seconds.
    pub sector1_time: f32,

    /// Time in sector 2 in seconds.
    pub sector2_time: f32,

    /// Distance the vehicle is around the current lap in meters. Can be
    /// negative if the car has not crossed the line yet (e.g. in qualifying).
    pub lap_distance: f32,

    /// Total distance traveled in session in meters. Can be negative if the car
    /// has not crossed the line yet (e.g. in qualifying).
    pub total_distance: f32,

    /// Delta in seconds for safety car.
    pub safety_car_delta: f32,

    /// Position of the car in the race.
    pub position: u8,

    /// Number of the current lap.
    pub current_lap_number: u8,

    /// Pit status.
    pub pit_status: PitStatus,

    /// Current sector.
    pub sector: Sector,

    /// Whether the current lap is valid.
    pub is_lap_valid: bool,

    /// Accumulated time penalties to be added in seconds.
    pub penalties: u8,

    /// Grid position the vehicle started the race in.
    pub grid_position: u8,

    /// Driver status.
    pub driver_status: DriverStatus,

    /// Result status.
    pub result_status: ResultStatus,
}

/// A packet with details about each driver's current lap.
///
/// F1 2019 publishes a lap packet with data in each driver in the session and their current status.
/// The packet contains information about their current lap, e.g. their position on track and
/// current lap time, but also data on the driver's best lap and their current status.
pub struct LapPacket {
    /// Each packet starts with a packet header.
    pub header: PacketHeader,

    /// Lap data for all cars on track.
    pub laps: Vec<Lap>,
}

impl TryFrom<u8> for DriverStatus {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(DriverStatus::InGarage),
            1 => Ok(DriverStatus::FlyingLap),
            2 => Ok(DriverStatus::InLap),
            3 => Ok(DriverStatus::OutLap),
            4 => Ok(DriverStatus::OnTrack),
            _ => Err(Error::new(
                ErrorKind::InvalidData,
                "Failed to decode driver status.",
            )),
        }
    }
}

impl TryFrom<u8> for PitStatus {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(PitStatus::None),
            1 => Ok(PitStatus::Pitting),
            2 => Ok(PitStatus::InPits),
            _ => Err(Error::new(
                ErrorKind::InvalidData,
                "Failed to decode pit status.",
            )),
        }
    }
}

impl TryFrom<u8> for ResultStatus {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ResultStatus::Invalid),
            1 => Ok(ResultStatus::Inactive),
            2 => Ok(ResultStatus::Active),
            3 => Ok(ResultStatus::Finished),
            4 => Ok(ResultStatus::Disqualified),
            5 => Ok(ResultStatus::NotClassified),
            6 => Ok(ResultStatus::Retired),
            _ => Err(Error::new(
                ErrorKind::InvalidData,
                "Failed to decode result status.",
            )),
        }
    }
}

impl TryFrom<u8> for Sector {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Sector::First),
            1 => Ok(Sector::Second),
            2 => Ok(Sector::Third),
            _ => Err(Error::new(
                ErrorKind::InvalidData,
                "Failed to decode sector.",
            )),
        }
    }
}

impl FromBytes for LapPacket {
    fn buffer_size() -> usize {
        843
    }

    fn decode(cursor: &mut Cursor<&mut BytesMut>) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let header = PacketHeader::decode(cursor)?;
        let mut laps = Vec::with_capacity(20);

        for _ in 0..20 {
            laps.push(Lap {
                last_lap_time: cursor.get_f32_le(),
                current_lap_time: cursor.get_f32_le(),
                best_lap_time: cursor.get_f32_le(),
                sector1_time: cursor.get_f32_le(),
                sector2_time: cursor.get_f32_le(),
                lap_distance: cursor.get_f32_le(),
                total_distance: cursor.get_f32_le(),
                safety_car_delta: cursor.get_f32_le(),
                position: cursor.get_u8(),
                current_lap_number: cursor.get_u8(),
                pit_status: PitStatus::try_from(cursor.get_u8())?,
                sector: Sector::try_from(cursor.get_u8())?,
                is_lap_valid: cursor.get_u8() > 1,
                penalties: cursor.get_u8(),
                grid_position: cursor.get_u8(),
                driver_status: DriverStatus::try_from(cursor.get_u8())?,
                result_status: ResultStatus::try_from(cursor.get_u8())?,
            });
        }

        Ok(LapPacket { header, laps })
    }
}
