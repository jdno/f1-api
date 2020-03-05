use crate::nineteen::{Flag, PacketHeader};
use crate::packet::FromBytes;
use bitflags::_core::convert::TryFrom;
use bytes::{Buf, BytesMut};
use std::io::{Cursor, Error, ErrorKind};

pub enum TractionControl {
    Off = 0,
    Low = 1,
    High = 2,
}

pub enum FuelMix {
    Lean = 0,
    Standard = 1,
    Rich = 2,
    Max = 3,
}

pub enum DrsStatus {
    Unknown = -1,
    NotAllowed = 0,
    Allowed = 1,
}

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

pub enum VisualCompound {
    Intermediate = 7,
    Wet = 8,
    Soft = 16,
    Medium = 17,
    Hard = 18,
}

pub enum ErsDeployMode {
    None = 0,
    Low = 1,
    Medium = 2,
    High = 3,
    Overtake = 4,
    Hotlap = 5,
}

pub struct CarStatus {
    /// Traction control setting.
    traction_control: TractionControl,

    /// Whether ABS is enabled.
    abs: bool,

    /// Fuel mix setting.
    fuel_mix: FuelMix,

    /// Front brake bias (percentage).
    brake_bias: u8,

    /// Whether the pit speed limiter is engaged.
    pit_limiter: bool,

    /// Remaining fuel mass in tank.
    fuel_remaining: f32,

    /// Fuel capacity.
    fuel_capacity: f32,

    /// Remaining fuel in terms of laps.
    fuel_remaining_laps: f32,

    /// The car's maximum RPM where the rev limiter kicks in.
    max_rpm: u16,

    /// The car's idle RPM.
    idle_rpm: u16,

    /// The car's number of gears.
    gear_count: u8,

    /// The status of DRS.
    drs: DrsStatus,

    /// The tyre wear at the RL, RR, FL, and FR in percent.
    tyre_wear: (u8, u8, u8, u8),

    /// The actual compound of the tyres.
    tyre_compound: TyreCompound,

    /// The visual compound of the tyres.
    visual_compound: VisualCompound,

    /// Tyre damage at the RL, RR, FL, and FR in percent.
    tyre_damage: (u8, u8, u8, u8),

    /// Damage to the left front wing in percent.
    front_left_wing_damage: u8,

    /// Damage to the right front wing in percent.
    front_right_wing_damage: u8,

    /// Rear wing damage in percent.
    rear_wing_damage: u8,

    /// Engine damage in percent.
    engine_damage: u8,

    /// Gear box damage in percent.
    gear_box_damage: u8,

    /// Flags shown to the current car.
    vehicle_flags: Flag,

    /// ERS energy store in Joules.
    ers_energy: f32,

    /// ERS deploy mode.
    ers_mode: ErsDeployMode,

    /// ERS energy harvested this lap by the MGU-K.
    ers_harvest_mguk: f32,

    /// ERS energy harvested this lap by the MGU-H.
    ers_harvest_mguh: f32,

    /// ERS energy deployed this lap.
    ers_deployed: f32,
}

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

    fn decode(cursor: &mut Cursor<BytesMut>) -> Result<Self, Error>
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
