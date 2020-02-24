use crate::nineteen::PacketHeader;
use bitflags::bitflags;

bitflags! {
    pub struct Button: u32 {
        const CROSS_OR_A = 0x001;
        const TRIANGLE_OR_Y = 0x0002;
        const CIRCLE_OR_B = 0x0004;
        const SQUARE_OR_X = 0x0008;
        const DPAD_LEFT = 0x0010;
        const DPAD_RIGHT = 0x0020;
        const DPAD_UP = 0x0040;
        const DPAD_DOWN = 0x0080;
        const OPTIONS_OR_MENU = 0x0100;
        const L1_OR_LB = 0x0200;
        const R1_OR_RB = 0x0400;
        const L2_OR_LT = 0x0800;
        const R2_OR_RT = 0x1000;
        const LEFT_STICK_CLICK = 0x2000;
        const RIGHT_STICK_CLICK =0x4000;
    }
}

pub enum Gear {
    Reverse = -1,
    Neutral = 0,
    First = 1,
    Second = 2,
    Third = 3,
    Fourth = 4,
    Fifth = 5,
    Sixth = 6,
    Seventh = 7,
    Eighth = 8,
}

pub enum Surface {
    Tarmac = 0,
    RumbleStrip = 1,
    Concrete = 2,
    Rock = 3,
    Gravel = 4,
    Mud = 5,
    Sand = 6,
    Grass = 7,
    Water = 8,
    Cobblestone = 9,
    Metal = 10,
    Ridged = 11,
}

pub struct Telemetry {
    /// The speed of the car in kilometers per hour.
    speed: u16,

    /// Ratio of applied throttle (0.0 to 1.0).
    throttle: f32,

    /// Ratio of steering input (-1.0 full lock left to 1.0 full lock right).
    steering: f32,

    /// Ratio of brake applied (0.0 to 1.0).
    brake: f32,

    /// Amount of clutch applied (0 to 100).
    clutch: u8,

    /// The current gear.
    gear: Gear,

    /// The engine's RPM.
    engine_rpm: u16,

    /// Whether the DRS is deployed.
    drs: bool,

    /// Rev lights indicator (percentage).
    rev_lights: u8,

    /// Brake temperature at the RL, RR, FL, FR in degrees celsius.
    brake_temperature: (u16, u16, u16, u16),

    /// Tyre surface temperature at the RL, RR, FL, FR in degrees celsius.
    tyre_surface_temperature: (u16, u16, u16, u16),

    /// Tyre inner temperature at the RL, RR, FL, FR in degrees celsius.
    tyre_inner_temperature: (u16, u16, u16, u16),

    /// Engine temperature in degrees celsius.
    engine_temperature: u16,

    /// Tyre pressure at the RL, RR, FL, FR in PSI.
    tyre_pressure: (f32, f32, f32, f32),

    /// The type of the surface the RL, RR, FL, and FR tyre have contact with.
    surface_type: (Surface, Surface, Surface, Surface),
}

pub struct TelemetryPacket {
    /// Each packet starts with a packet header.
    pub header: PacketHeader,

    /// The telemetry data for each car in the session.
    pub telemetry: Vec<Telemetry>,

    /// Bit flag indicating which buttons are currently pressed.
    pub button_status: Button,
}
