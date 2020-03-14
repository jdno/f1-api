//! Packet with the setups of all cars in the session

use crate::nineteen::PacketHeader;
use crate::packet::FromBytes;
use bytes::{Buf, BytesMut};
use std::io::{Cursor, Error};

/// A setup of an F1 race car.
///
/// The setup of the race cars in F1 2019 can be modified to improve the handling of the car on a
/// particular race track. The setup consists of a set of parameters that can be adjusted.
pub struct CarSetup {
    /// Front wing aero.
    pub front_wing: u8,

    /// Rear wing aero.
    pub rear_wing: u8,

    /// Differential adjustment on throttle (percentage).
    pub on_throttle: u8,

    /// Differential adjustment off throttle (percentage).
    pub off_throttle: u8,

    /// Front camber angle (suspension geometry).
    pub front_camber: f32,

    /// Rear camber angle (suspension geometry).
    pub rear_camber: f32,

    /// Front toe angle (suspension geometry).
    pub front_toe: f32,

    /// Rear toe angle (suspension geometry).
    pub rear_toe: f32,

    /// Front suspension setting.
    pub front_suspension: u8,

    /// Rear suspension setting.
    pub rear_suspension: u8,

    /// Front anti-roll bar.
    pub front_anti_roll_bar: u8,

    /// Rear anti-roll bar.
    pub rear_anti_roll_bar: u8,

    /// Front ride height.
    pub front_suspension_height: u8,

    /// Rear right height.
    pub rear_suspension_height: u8,

    /// Brake pressure (percentage).
    pub brake_pressure: u8,

    /// Brake bias (percentage).
    pub brake_bias: u8,

    /// Front tyre pressure (PSI).
    pub front_tyre_pressure: f32,

    /// Rear tyre pressure (PSI).
    pub rear_tyre_pressure: f32,

    /// Ballast.
    pub ballast: u8,

    /// Fuel load.
    pub fuel_load: f32,
}

/// A packet with the setups of all cars in the session.
///
/// F1 2019 publishes the setups of all cars on track. In multiplayer sessions, the setups of other
/// players will appear.
pub struct CarSetupPacket {
    /// Each packet starts with a packet header.
    pub header: PacketHeader,

    /// The setup for each car in the session. In multiplayer sessions, the
    /// setup for the cars of other players will appear empty.
    pub setups: Vec<CarSetup>,
}

impl FromBytes for CarSetupPacket {
    fn buffer_size() -> usize {
        843
    }

    fn decode(cursor: &mut Cursor<&mut BytesMut>) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let header = PacketHeader::decode(cursor)?;
        let mut setups = Vec::with_capacity(20);

        for _ in 0..20 {
            setups.push(CarSetup {
                front_wing: cursor.get_u8(),
                rear_wing: cursor.get_u8(),
                on_throttle: cursor.get_u8(),
                off_throttle: cursor.get_u8(),
                front_camber: cursor.get_f32_le(),
                rear_camber: cursor.get_f32_le(),
                front_toe: cursor.get_f32_le(),
                rear_toe: cursor.get_f32_le(),
                front_suspension: cursor.get_u8(),
                rear_suspension: cursor.get_u8(),
                front_anti_roll_bar: cursor.get_u8(),
                rear_anti_roll_bar: cursor.get_u8(),
                front_suspension_height: cursor.get_u8(),
                rear_suspension_height: cursor.get_u8(),
                brake_pressure: cursor.get_u8(),
                brake_bias: cursor.get_u8(),
                front_tyre_pressure: cursor.get_f32_le(),
                rear_tyre_pressure: cursor.get_f32_le(),
                ballast: cursor.get_u8(),
                fuel_load: cursor.get_f32_le(),
            })
        }

        Ok(CarSetupPacket { header, setups })
    }
}
