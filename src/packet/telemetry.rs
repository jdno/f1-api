//! Telemetry data coming from a car
//!
//! The F1 games publish telemetry data about each car at a configurable rate. The telemetry data
//! includes physical properties of the car, e.g. its speed, but also information about the controls
//! that are applied, e.g. which buttons are being pressed.

use crate::packet::header::Header;
use crate::types::CornerProperty;
use bitflags::bitflags;
use derive_new::new;
use getset::{CopyGetters, Getters};

bitflags! {
    /// A bit field with currently pressed buttons.
    ///
    /// The F1 games publish which buttons are currently being pressed by the user. This information
    /// is encoded in a bit field, where each bit represents a different button.
    pub struct Button: u32 {
        const NONE = 0x0;
        const CROSS_OR_A = 0x0001;
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

impl Default for Button {
    fn default() -> Self {
        Button::NONE
    }
}

/// Gears of a Formula One car
#[derive(Debug, PartialEq, Copy, Clone, Eq, Ord, PartialOrd, Hash)]
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

impl Default for Gear {
    fn default() -> Self {
        Gear::Neutral
    }
}

/// Surfaces that a tyre can come in contact with in the F1 games
#[derive(Debug, PartialEq, Copy, Clone, Eq, Ord, PartialOrd, Hash)]
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

impl Default for Surface {
    fn default() -> Self {
        Surface::Tarmac
    }
}

/// Telemetry data coming from a car
///
/// The telemetry data provided from the F1 games contains detailed, and quickly changing data on
/// the inner mechanics of each car, e.g. its speed, engine RPMs, and temperatures.
#[derive(new, Debug, CopyGetters, Getters, PartialEq, Copy, Clone, PartialOrd, Default)]
#[allow(clippy::too_many_arguments)]
pub struct Telemetry {
    /// Returns the speed of the car in kilometers per hour.
    #[getset(get_copy = "pub")]
    speed: u16,

    /// Returns the ratio of the applied throttle.
    #[getset(get_copy = "pub")]
    throttle: f32,

    /// Returns the ratio of steering input.
    ///
    /// The values range from -1.0 for a full lock left to 1.0 for a full lock right.
    #[getset(get_copy = "pub")]
    steering: f32,

    /// Returns the ratio of brake applied.
    #[getset(get_copy = "pub")]
    brake: f32,

    /// Returns the percentage that the clutch has been applied.
    #[getset(get_copy = "pub")]
    clutch: u8,

    /// Returns the gear the car is in.
    #[getset(get_copy = "pub")]
    gear: Gear,

    /// Returns the engine RPM.
    #[getset(get_copy = "pub")]
    engine_rpm: u16,

    /// Returns whether the DRS is deployed.
    #[getset(get_copy = "pub")]
    drs: bool,

    /// Returns the percentage of how far the rev lights indicator is engaged.
    #[getset(get_copy = "pub")]
    rev_lights: u8,

    /// Returns the brake temperature at each corner of the in degrees celsius.
    #[getset(get = "pub")]
    brake_temperature: CornerProperty<u16>,

    /// Returns the tyre surface temperature at each corner of the car in degrees celsius.
    #[getset(get = "pub")]
    tyre_surface_temperature: CornerProperty<u16>,

    /// Returns the tyre inner temperature at each corner of the car in degrees celsius.
    #[getset(get = "pub")]
    tyre_inner_temperature: CornerProperty<u16>,

    /// Returns the engine temperature in degrees celsius.
    #[getset(get_copy = "pub")]
    engine_temperature: u16,

    /// Returns the tyre pressure at each corner of the car in psi.
    #[getset(get = "pub")]
    tyre_pressure: CornerProperty<f32>,

    /// Returns the type of the surface each tyre fo the car has contact with.
    #[getset(get = "pub")]
    surface_type: CornerProperty<Surface>,
}

/// Packet containing the telemetry of all cars in the session
///
/// The F1 games publish telemetry data for each car in the session. The telemetry data includes
/// parameters such as the car's speed, as well as information in controller inputs from the user.
#[derive(new, Debug, CopyGetters, Getters, PartialEq, Clone, PartialOrd)]
pub struct TelemetryPacket {
    /// Returns the packet header prefixing the telemetry packet.
    #[getset(get = "pub")]
    header: Header,

    /// Returns the telemetry data for each car in the session.
    #[getset(get = "pub")]
    telemetry: Vec<Telemetry>,

    /// Returns a bit flag indicating which buttons are currently pressed.
    #[getset(get_copy = "pub")]
    button_status: Button,
}
