//! Packet with the status of each car in the session

use crate::from_bytes::FromBytes;
use crate::nineteen::{Flag, PacketHeader};
use bitflags::_core::convert::TryFrom;
use bytes::{Buf, BytesMut};
use std::io::{Cursor, Error, ErrorKind};

/// Traction control.
pub enum TractionControl {
    Off = 0,
    Low = 1,
    High = 2,
}

/// Fuel mix.
pub enum FuelMix {
    Lean = 0,
    Standard = 1,
    Rich = 2,
    Max = 3,
}

/// DRS.
pub enum DrsStatus {
    Unknown = -1,
    NotAllowed = 0,
    Allowed = 1,
}

/// The actual/physical tyre compound.
///
/// F1 2019 supports different tyre compounds. The game differentiates between the actual/physical
/// tyre compounds that influence the handling of the car, and purely visual ones that simply change
/// the look of the tyre. All types of formula racing (modern F1, F2, classic F1) share the same
/// virtual compounds, but have different actual compounds.
pub enum TyreCompound {
    F1Intermediate = 7,
    F1Wet = 8,
    ClassicDry = 9,
    ClassicWet = 10,
    F2SuperSoft = 11,
    F2Soft = 12,
    F2Medium = 13,
    F2Hard = 14,
    F2Wet = 15,
    F1C5 = 16,
    F1C4 = 17,
    F1C3 = 18,
    F1C2 = 19,
    F1C1 = 20,
}

/// The virtual tyre compound.
///
/// F1 2019 supports different tyre compounds. The game differentiates between the actual/physical
/// tyre compounds that influence the handling of the car, and purely visual ones that simply change
/// the look of the tyre. All types of formula racing (modern F1, F2, classic F1) share the same
/// virtual compounds, but have different actual compounds.
pub enum VisualCompound {
    Intermediate = 7,
    Wet = 8,
    Soft = 16,
    Medium = 17,
    Hard = 18,
}

/// ERS deploy mode.
pub enum ErsDeployMode {
    None = 0,
    Low = 1,
    Medium = 2,
    High = 3,
    Overtake = 4,
    Hotlap = 5,
}

/// The status of a race car.
///
/// The status of each car is a collection of properties that can change over time. It includes data
/// about the fuel, the engine, the various assistance systems like ABS, DRS, and ERS, and the
/// damage the car has sustained.
pub struct CarStatus {
    /// Traction control setting.
    pub traction_control: TractionControl,

    /// Whether ABS is enabled.
    pub abs: bool,

    /// Fuel mix setting.
    pub fuel_mix: FuelMix,

    /// Front brake bias (percentage).
    pub brake_bias: u8,

    /// Whether the pit speed limiter is engaged.
    pub pit_limiter: bool,

    /// Remaining fuel mass in tank.
    pub fuel_remaining: f32,

    /// Fuel capacity.
    pub fuel_capacity: f32,

    /// Remaining fuel in terms of laps.
    pub fuel_remaining_laps: f32,

    /// The car's maximum RPM where the rev limiter kicks in.
    pub max_rpm: u16,

    /// The car's idle RPM.
    pub idle_rpm: u16,

    /// The car's number of gears.
    pub gear_count: u8,

    /// The status of DRS.
    pub drs: DrsStatus,

    /// The tyre wear at the RL, RR, FL, and FR in percent.
    pub tyre_wear: (u8, u8, u8, u8),

    /// The actual compound of the tyres.
    pub tyre_compound: TyreCompound,

    /// The visual compound of the tyres.
    pub visual_compound: VisualCompound,

    /// Tyre damage at the RL, RR, FL, and FR in percent.
    pub tyre_damage: (u8, u8, u8, u8),

    /// Damage to the left front wing in percent.
    pub front_left_wing_damage: u8,

    /// Damage to the right front wing in percent.
    pub front_right_wing_damage: u8,

    /// Rear wing damage in percent.
    pub rear_wing_damage: u8,

    /// Engine damage in percent.
    pub engine_damage: u8,

    /// Gear box damage in percent.
    pub gear_box_damage: u8,

    /// Flags shown to the current car.
    pub vehicle_flags: Flag,

    /// ERS energy store in Joules.
    pub ers_energy: f32,

    /// ERS deploy mode.
    pub ers_mode: ErsDeployMode,

    /// ERS energy harvested this lap by the MGU-K.
    pub ers_harvest_mguk: f32,

    /// ERS energy harvested this lap by the MGU-H.
    pub ers_harvest_mguh: f32,

    /// ERS energy deployed this lap.
    pub ers_deployed: f32,
}

/// A packet with the status of each car in the session.
pub struct CarStatusPacket {
    /// Each packet starts with a packet header.
    pub header: PacketHeader,

    /// The status of each car in the session.
    pub car_status: Vec<CarStatus>,
}

impl TryFrom<u8> for TractionControl {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(TractionControl::Off),
            1 => Ok(TractionControl::Low),
            2 => Ok(TractionControl::High),
            _ => Err(Error::new(
                ErrorKind::InvalidData,
                "Failed to decode transaction control.",
            )),
        }
    }
}

impl TryFrom<u8> for FuelMix {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(FuelMix::Lean),
            1 => Ok(FuelMix::Standard),
            2 => Ok(FuelMix::Rich),
            3 => Ok(FuelMix::Max),
            _ => Err(Error::new(
                ErrorKind::InvalidData,
                "Failed to decode fuel mix.",
            )),
        }
    }
}

impl TryFrom<i8> for DrsStatus {
    type Error = Error;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        match value {
            -1 => Ok(DrsStatus::Unknown),
            0 => Ok(DrsStatus::NotAllowed),
            1 => Ok(DrsStatus::Allowed),
            _ => Err(Error::new(
                ErrorKind::InvalidData,
                "Failed to decode DRS status.",
            )),
        }
    }
}

impl TryFrom<u8> for TyreCompound {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            7 => Ok(TyreCompound::F1Intermediate),
            8 => Ok(TyreCompound::F1Wet),
            9 => Ok(TyreCompound::ClassicDry),
            10 => Ok(TyreCompound::ClassicWet),
            11 => Ok(TyreCompound::F2SuperSoft),
            12 => Ok(TyreCompound::F2Soft),
            13 => Ok(TyreCompound::F2Medium),
            14 => Ok(TyreCompound::F2Hard),
            15 => Ok(TyreCompound::F2Wet),
            16 => Ok(TyreCompound::F1C5),
            17 => Ok(TyreCompound::F1C4),
            18 => Ok(TyreCompound::F1C3),
            19 => Ok(TyreCompound::F1C2),
            20 => Ok(TyreCompound::F1C1),
            _ => Err(Error::new(
                ErrorKind::InvalidData,
                "Failed to decode tyre compound.",
            )),
        }
    }
}

impl TryFrom<u8> for VisualCompound {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            7 => Ok(VisualCompound::Intermediate),
            8 => Ok(VisualCompound::Wet),
            16 => Ok(VisualCompound::Soft),
            17 => Ok(VisualCompound::Medium),
            18 => Ok(VisualCompound::Hard),
            _ => Err(Error::new(
                ErrorKind::InvalidData,
                "Failed to decode visual tyre compound.",
            )),
        }
    }
}

impl TryFrom<u8> for ErsDeployMode {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ErsDeployMode::None),
            1 => Ok(ErsDeployMode::Low),
            2 => Ok(ErsDeployMode::Medium),
            3 => Ok(ErsDeployMode::High),
            4 => Ok(ErsDeployMode::Overtake),
            5 => Ok(ErsDeployMode::Hotlap),
            _ => Err(Error::new(
                ErrorKind::InvalidData,
                "Failed to decode ERS deployment mode.",
            )),
        }
    }
}

impl FromBytes for CarStatusPacket {
    fn buffer_size() -> usize {
        1143
    }

    fn decode(cursor: &mut Cursor<&mut BytesMut>) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let header = PacketHeader::decode(cursor)?;
        let mut car_status = Vec::with_capacity(20);

        for _ in 0..20 {
            car_status.push(CarStatus {
                traction_control: TractionControl::try_from(cursor.get_u8())?,
                abs: cursor.get_u8() > 0,
                fuel_mix: FuelMix::try_from(cursor.get_u8())?,
                brake_bias: cursor.get_u8(),
                pit_limiter: cursor.get_u8() > 0,
                fuel_remaining: cursor.get_f32_le(),
                fuel_capacity: cursor.get_f32_le(),
                fuel_remaining_laps: cursor.get_f32_le(),
                max_rpm: cursor.get_u16_le(),
                idle_rpm: cursor.get_u16_le(),
                gear_count: cursor.get_u8(),
                drs: DrsStatus::try_from(cursor.get_i8())?,
                tyre_wear: (
                    cursor.get_u8(),
                    cursor.get_u8(),
                    cursor.get_u8(),
                    cursor.get_u8(),
                ),
                tyre_compound: TyreCompound::try_from(cursor.get_u8())?,
                visual_compound: VisualCompound::try_from(cursor.get_u8())?,
                tyre_damage: (
                    cursor.get_u8(),
                    cursor.get_u8(),
                    cursor.get_u8(),
                    cursor.get_u8(),
                ),
                front_left_wing_damage: cursor.get_u8(),
                front_right_wing_damage: cursor.get_u8(),
                rear_wing_damage: cursor.get_u8(),
                engine_damage: cursor.get_u8(),
                gear_box_damage: cursor.get_u8(),
                vehicle_flags: Flag::try_from(cursor.get_i8())?,
                ers_energy: cursor.get_f32_le(),
                ers_mode: ErsDeployMode::try_from(cursor.get_u8())?,
                ers_harvest_mguk: cursor.get_f32_le(),
                ers_harvest_mguh: cursor.get_f32_le(),
                ers_deployed: cursor.get_f32_le(),
            });
        }

        Ok(CarStatusPacket { header, car_status })
    }
}
