//! Data on the status of each car in the session
//!
//! The F1 games provide detailed information about the status of each car in the session. The rate
//! with which the data is provided can be configured in the in-game settings.

use crate::packet::header::Header;
use crate::types::{CornerProperty, Flag};
use derive_new::new;
use getset::{CopyGetters, Getters};

/// Traction control settings
///
/// Traction control is a driver assist that does only exist in-game, and not on an actual F1 car.
/// It can be turned off, or switched between a low and high setting.
#[derive(Debug, PartialEq, Copy, Clone, Eq, Ord, PartialOrd, Hash)]
pub enum TractionControl {
    /// Traction control is turned off.
    Off,

    /// Traction control operates at a low setting, and offers only minor assists.
    Low,

    /// Traction control operates at a high setting, and offers maximum help.
    High,
}

impl Default for TractionControl {
    fn default() -> Self {
        TractionControl::Off
    }
}

/// Fuel mix settings
///
/// F1 cars can run on different fuel mixes, and drivers are often required to change the fuel mix
/// during a race to save fuel or prevent the engine from overheating.
#[derive(Debug, PartialEq, Copy, Clone, Eq, Ord, PartialOrd, Hash)]
pub enum FuelMix {
    /// The engine runs on a lean fuel mix.
    Lean,

    /// The engine runs on the standard fuel mix.
    Standard,

    /// The engine runs on a rich fuel mix.
    Rich,

    /// The engine runs on the richest fuel mix.
    Max,
}

impl Default for FuelMix {
    fn default() -> Self {
        FuelMix::Standard
    }
}

/// Setting of the Drag Reduction System
///
/// The Drag Reduction System, or DRS, can be disabled and enabled during a race. When it is
/// disabled, drivers cannot activate it.
#[derive(Debug, PartialEq, Copy, Clone, Eq, Ord, PartialOrd, Hash)]
pub enum DrsSetting {
    ///  The DRS setting is unknown, for example because the current formula does not support it.
    Unknown,

    /// DRS is disabled, and cannot be used by drivers.
    NotAllowed,

    /// DRS is enabled, and can be used by drivers.
    Allowed,
}

impl Default for DrsSetting {
    fn default() -> Self {
        DrsSetting::Unknown
    }
}

/// Tyre compounds that influence the physical simulation
///
/// The latest generations of F1 games started to distinguish between physical and visual tyre
/// compounds to support Pirelli's system with five tyre compounds from which three are picked for a
/// race weekend and then labeled soft, medium, and hard. The physical tyre compound describes which
/// compound is used, while the visual tyre compound indicates if it is a soft, medium, or hard
/// tyre.
///
/// For older games that do not know this distinction yet, the tyre compound is duplicated in both
/// fields.
#[derive(Debug, PartialEq, Copy, Clone, Eq, Ord, PartialOrd, Hash)]
pub enum PhysicalTyreCompound {
    ClassicDry,
    ClassicWet,
    F1C1,
    F1C2,
    F1C3,
    F1C4,
    F1C5,
    F1HyperSoft,
    F1UltraSoft,
    F1SuperSoft,
    F1Soft,
    F1Medium,
    F1Hard,
    F1SuperHard,
    F1Intermediate,
    F1Wet,
    F2SuperSoft,
    F2Soft,
    F2Medium,
    F2Hard,
    F2Wet,
}

impl Default for PhysicalTyreCompound {
    fn default() -> Self {
        PhysicalTyreCompound::F1C1
    }
}

/// Tyre compounds that influence the visual appearance
///
/// The latest generations of F1 games started to distinguish between physical and visual tyre
/// compounds to support Pirelli's system with five tyre compounds from which three are picked for a
/// race weekend and then labeled soft, medium, and hard. The physical tyre compound describes which
/// compound is used, while the visual tyre compound indicates if it is a soft, medium, or hard
/// tyre.
///
/// For older games that do not know this distinction yet, the tyre compound is duplicated in both
/// fields.
#[derive(Debug, PartialEq, Copy, Clone, Eq, Ord, PartialOrd, Hash)]
pub enum VisualTyreCompound {
    ClassicDry,
    ClassicWet,
    F1HyperSoft,
    F1UltraSoft,
    F1SuperSoft,
    F1Soft,
    F1Medium,
    F1Hard,
    F1SuperHard,
    F1Intermediate,
    F1Wet,
    F2SuperSoft,
    F2Soft,
    F2Medium,
    F2Hard,
    F2Wet,
}

impl Default for VisualTyreCompound {
    fn default() -> Self {
        VisualTyreCompound::F1Soft
    }
}

/// Deploy modes for the Energy Recovery System
///
/// The Energy Recovery System, or ERS, can be operated in different modes that determine how much
/// energy is harvested under braking, and how much is used to accelerate the car.
#[derive(Debug, PartialEq, Copy, Clone, Eq, Ord, PartialOrd, Hash)]
pub enum ErsDeployMode {
    /// The Energy Recovery System is disabled or does not exist in the current car.
    None,

    /// The Energy Recovery System operates at a low setting, and harvest more energy than it
    /// consumes.
    Low,

    /// The Energy Recovery System operates at a medium setting, harvesting and deploying at a
    /// similar rate.
    Medium,

    /// The Energy Recovery System operates at a high setting, and deploys more energy than it can
    /// harvest.
    High,

    /// The Energy Recovery System operates at a very high level, and deploys far more energy than
    /// it can harvest.
    Overtake,

    /// The Energy Recovery System operates at a level optimized for qualifying laps, which aims to
    /// deplete the batteries by the end of the lap.
    Hotlap,
}

impl Default for ErsDeployMode {
    fn default() -> Self {
        ErsDeployMode::Low
    }
}

/// Data describing the status of a car
///
/// The status of each car is a collection of properties that can change over time. It includes data
/// about the fuel, the engine, the various assistance systems like ABS, DRS, and ERS, and the
/// damage the car has sustained. In multiplayer sessions, some of this data is restricted and only
/// shown for the player's own car.
#[derive(new, Debug, CopyGetters, Getters, PartialEq, Copy, Clone, PartialOrd, Default)]
#[allow(clippy::too_many_arguments)]
pub struct CarStatus {
    /// Returns the traction control setting.
    #[getset(get_copy = "pub")]
    traction_control: TractionControl,

    /// Returns whether ABS is enabled.
    #[getset(get_copy = "pub")]
    abs: bool,

    /// Returns the fuel mix setting.
    #[getset(get_copy = "pub")]
    fuel_mix: FuelMix,

    /// Returns the front brake bias (percentage).
    #[getset(get_copy = "pub")]
    brake_bias: u8,

    /// Returns whether the pit speed limiter is engaged.
    #[getset(get_copy = "pub")]
    pit_limiter: bool,

    /// Returns the remaining fuel mass in tank.
    #[getset(get_copy = "pub")]
    fuel_remaining: f32,

    /// Returns the fuel capacity.
    #[getset(get_copy = "pub")]
    fuel_capacity: f32,

    /// Returns the remaining fuel in terms of laps.
    #[getset(get_copy = "pub")]
    fuel_remaining_laps: f32,

    /// Returns the car's maximum RPM where the rev limiter kicks in.
    #[getset(get_copy = "pub")]
    max_rpm: u16,

    /// Returns the car's idle RPM.
    #[getset(get_copy = "pub")]
    idle_rpm: u16,

    /// Returns the car's number of gears.
    #[getset(get_copy = "pub")]
    gear_count: u8,

    /// Returns the status of DRS.
    #[getset(get_copy = "pub")]
    drs: DrsSetting,

    /// Returns the tyre wear at each corner of the car in percent.
    #[getset(get = "pub")]
    tyre_wear: CornerProperty<u8>,

    /// Returns the physical compound of the tyres.
    #[getset(get_copy = "pub")]
    physical_tyre_compound: PhysicalTyreCompound,

    /// Returns the visual compound of the tyres.
    #[getset(get_copy = "pub")]
    visual_tyre_compound: VisualTyreCompound,

    /// Returns the tyre damage at each corner of the car in percent.
    #[getset(get = "pub")]
    tyre_damage: CornerProperty<u8>,

    /// Returns the damage to the left front wing in percent.
    #[getset(get_copy = "pub")]
    front_left_wing_damage: u8,

    /// Returns the damage to the right front wing in percent.
    #[getset(get_copy = "pub")]
    front_right_wing_damage: u8,

    /// Returns the damage to the rear wing in percent.
    #[getset(get_copy = "pub")]
    rear_wing_damage: u8,

    /// Returns the damage to the engine in percent.
    #[getset(get_copy = "pub")]
    engine_damage: u8,

    /// Returns the damage to the gear box in percent.
    #[getset(get_copy = "pub")]
    gear_box_damage: u8,

    /// Returns the flags that are being shown to the current car.
    #[getset(get_copy = "pub")]
    vehicle_flags: Flag,

    /// Returns the ERS energy store in Joules.
    #[getset(get_copy = "pub")]
    ers_energy: f32,

    /// Returns the ERS deploy mode.
    #[getset(get_copy = "pub")]
    ers_deploy_mode: ErsDeployMode,

    /// Returns the ERS energy harvested this lap by the MGU-K.
    #[getset(get_copy = "pub")]
    ers_harvest_mgu_k: f32,

    /// Returns the ERS energy harvested this lap by the MGU-H.
    #[getset(get_copy = "pub")]
    ers_harvest_mgu_h: f32,

    /// Returns the ERS energy deployed this lap.
    #[getset(get_copy = "pub")]
    ers_deployed: f32,
}

/// Packet containing the status of each car in the session
///
/// The F1 games publish data on the status of each car in the session at a rate that can be
/// configured in the in-game settings.
#[derive(new, Debug, Getters, PartialEq, Clone, PartialOrd)]
pub struct CarStatusPacket {
    /// Returns the packet header prefixing the car status packet.
    #[getset(get = "pub")]
    header: Header,

    /// Returns the status of each car in the session.
    #[getset(get = "pub")]
    statuses: Vec<CarStatus>,
}
