use crate::nineteen::{Flag, PacketHeader};

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
