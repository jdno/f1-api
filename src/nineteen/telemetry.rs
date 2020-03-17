//! Packet with the telemetry data of each car in the session

use crate::from_bytes::FromBytes;
use crate::nineteen::PacketHeader;
use bitflags::bitflags;
use bytes::{Buf, BytesMut};
use std::convert::TryFrom;
use std::io::{Cursor, Error, ErrorKind};

bitflags! {
    /// A bit field with currently pressed buttons.
    ///
    /// F1 2019 publishes which buttons are currently being pressed by the user. This information is
    /// encoded in a bit field, where each bit represents a different button.
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

/// The gear the car is in.
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

/// The surface areas a tyre can be in contact with.
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

/// The telemetry of a race car.
///
/// Telemetry provides detailed, and quickly changing data on the inner mechanics of each car, e.g.
/// its speed, engine RPMs, and temperatures.
pub struct Telemetry {
    /// The speed of the car in kilometers per hour.
    pub speed: u16,

    /// Ratio of applied throttle (0.0 to 1.0).
    pub throttle: f32,

    /// Ratio of steering input (-1.0 full lock left to 1.0 full lock right).
    pub steering: f32,

    /// Ratio of brake applied (0.0 to 1.0).
    pub brake: f32,

    /// Amount of clutch applied (0 to 100).
    pub clutch: u8,

    /// The current gear.
    pub gear: Gear,

    /// The engine's RPM.
    pub engine_rpm: u16,

    /// Whether the DRS is deployed.
    pub drs: bool,

    /// Rev lights indicator (percentage).
    pub rev_lights: u8,

    /// Brake temperature at the RL, RR, FL, FR in degrees celsius.
    pub brake_temperature: (u16, u16, u16, u16),

    /// Tyre surface temperature at the RL, RR, FL, FR in degrees celsius.
    pub tyre_surface_temperature: (u16, u16, u16, u16),

    /// Tyre inner temperature at the RL, RR, FL, FR in degrees celsius.
    pub tyre_inner_temperature: (u16, u16, u16, u16),

    /// Engine temperature in degrees celsius.
    pub engine_temperature: u16,

    /// Tyre pressure at the RL, RR, FL, FR in PSI.
    pub tyre_pressure: (f32, f32, f32, f32),

    /// The type of the surface the RL, RR, FL, and FR tyre have contact with.
    pub surface_type: (Surface, Surface, Surface, Surface),
}

/// A packet with telemetry data for each car in the session.
pub struct TelemetryPacket {
    /// Each packet starts with a packet header.
    pub header: PacketHeader,

    /// The telemetry data for each car in the session.
    pub telemetry: Vec<Telemetry>,

    /// Bit flag indicating which buttons are currently pressed.
    pub button_status: Button,
}

impl TryFrom<i8> for Gear {
    type Error = Error;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        match value {
            -1 => Ok(Gear::Reverse),
            0 => Ok(Gear::Neutral),
            1 => Ok(Gear::First),
            2 => Ok(Gear::Second),
            3 => Ok(Gear::Third),
            4 => Ok(Gear::Fourth),
            5 => Ok(Gear::Fifth),
            6 => Ok(Gear::Sixth),
            7 => Ok(Gear::Seventh),
            8 => Ok(Gear::Eighth),
            _ => Err(Error::new(ErrorKind::InvalidData, "Failed to decode gear.")),
        }
    }
}

impl TryFrom<u8> for Surface {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Surface::Tarmac),
            1 => Ok(Surface::RumbleStrip),
            2 => Ok(Surface::Concrete),
            3 => Ok(Surface::Rock),
            4 => Ok(Surface::Gravel),
            5 => Ok(Surface::Mud),
            6 => Ok(Surface::Sand),
            7 => Ok(Surface::Grass),
            8 => Ok(Surface::Water),
            9 => Ok(Surface::Cobblestone),
            10 => Ok(Surface::Metal),
            11 => Ok(Surface::Ridged),
            _ => Err(Error::new(
                ErrorKind::InvalidData,
                "Failed to decode surface.",
            )),
        }
    }
}

impl FromBytes for TelemetryPacket {
    fn buffer_size() -> usize {
        1347
    }

    fn decode(cursor: &mut Cursor<&mut BytesMut>) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let header = PacketHeader::decode(cursor)?;
        let mut telemetry = Vec::with_capacity(20);

        for _ in 0..20 {
            telemetry.push(Telemetry {
                speed: cursor.get_u16_le(),
                throttle: cursor.get_f32_le(),
                steering: cursor.get_f32_le(),
                brake: cursor.get_f32_le(),
                clutch: cursor.get_u8(),
                gear: Gear::try_from(cursor.get_i8())?,
                engine_rpm: cursor.get_u16_le(),
                drs: cursor.get_u8() > 0,
                rev_lights: cursor.get_u8(),
                brake_temperature: (
                    cursor.get_u16_le(),
                    cursor.get_u16_le(),
                    cursor.get_u16_le(),
                    cursor.get_u16_le(),
                ),
                tyre_surface_temperature: (
                    cursor.get_u16_le(),
                    cursor.get_u16_le(),
                    cursor.get_u16_le(),
                    cursor.get_u16_le(),
                ),
                tyre_inner_temperature: (
                    cursor.get_u16_le(),
                    cursor.get_u16_le(),
                    cursor.get_u16_le(),
                    cursor.get_u16_le(),
                ),
                engine_temperature: cursor.get_u16_le(),
                tyre_pressure: (
                    cursor.get_f32_le(),
                    cursor.get_f32_le(),
                    cursor.get_f32_le(),
                    cursor.get_f32_le(),
                ),
                surface_type: (
                    Surface::try_from(cursor.get_u8())?,
                    Surface::try_from(cursor.get_u8())?,
                    Surface::try_from(cursor.get_u8())?,
                    Surface::try_from(cursor.get_u8())?,
                ),
            });
        }

        let button_status = match Button::from_bits(cursor.get_u32_le()) {
            Some(button) => button,
            None => Button { bits: 0 },
        };

        Ok(TelemetryPacket {
            header,
            telemetry,
            button_status,
        })
    }
}
