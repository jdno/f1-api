use crate::nineteen::PacketHeader;
use crate::packet::FromBytes;
use bytes::{Buf, BytesMut};
use std::convert::TryFrom;
use std::io::{Cursor, Error, ErrorKind};

pub enum DriverStatus {
    InGarage = 0,
    FlyingLap = 1,
    InLap = 2,
    OutLap = 3,
    OnTrack = 4,
}

pub enum PitStatus {
    None = 0,
    Pitting = 1,
    InPits = 2,
}

pub enum ResultStatus {
    Invalid = 0,
    Inactive = 1,
    Active = 2,
    Finished = 3,
    Disqualified = 4,
    NotClassified = 5,
    Retired = 6,
}

pub enum Sector {
    First = 0,
    Second = 1,
    Third = 2,
}

pub struct Lap {
    /// Last lap time in seconds.
    last_lap_time: f32,

    /// Current time around the lap in seconds.
    curent_lap_time: f32,

    /// Best lap time in the session in seconds.
    best_lap_time: f32,

    /// Time in sector 1 in seconds.
    sector1_time: f32,

    /// Time in sector 2 in seconds.
    sector2_time: f32,

    /// Distance the vehicle is around the current lap in meters. Can be
    /// negative if the car has not crossed the line yet (e.g. in qualifying).
    lap_distance: f32,

    /// Total distance traveled in session in meters. Can be negative if the car
    /// has not crossed the line yet (e.g. in qualifying).
    total_distance: f32,

    /// Delta in seconds for safety car.
    safety_car_delta: f32,

    /// Position of the car in the race.
    position: u8,

    /// Number of the current lap.
    current_lap_number: u8,

    /// Pit status.
    pit_status: PitStatus,

    /// Current sector.
    sector: Sector,

    /// Whether the current lap is valid.
    is_lap_valid: bool,

    /// Accumulated time penalties to be added in seconds.
    penalties: u8,

    /// Grid position the vehicle started the race in.
    grid_position: u8,

    /// Driver status.
    driver_status: DriverStatus,

    /// Result status.
    result_status: ResultStatus,
}

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

    fn decode(cursor: &mut Cursor<BytesMut>) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let header = PacketHeader::decode(cursor)?;
        let mut laps = Vec::with_capacity(20);

        for _ in 0..20 {
            laps.push(Lap {
                last_lap_time: cursor.get_f32_le(),
                curent_lap_time: cursor.get_f32_le(),
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
