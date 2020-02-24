use crate::nineteen::PacketHeader;

pub struct CarSetup {
    /// Front wing aero.
    front_wing: u8,

    /// Rear wing aero.
    rear_wing: u8,

    /// Differential adjustment on throttle (percentage).
    on_throttle: u8,

    /// Differential adjustment off throttle (percentage).
    off_throttle: u8,

    /// Front camber angle (suspension geometry).
    front_camber: f32,

    /// Rear camber angle (suspension geometry).
    rear_camber: f32,

    /// Front toe angle (suspension geometry).
    front_toe: f32,

    /// Rear toe angle (suspension geometry).
    rear_toe: f32,

    /// Front suspension setting.
    front_suspension: u8,

    /// Rear suspension setting.
    rear_suspension: u8,

    /// Front anti-roll bar.
    front_anti_roll_bar: u8,

    /// Rear anti-roll bar.
    rear_anti_roll_bar: u8,

    /// Front ride height.
    front_suspension_height: u8,

    /// Rear right height.
    rear_suspension_height: u8,

    /// Brake pressure (percentage).
    brake_pressure: u8,

    /// Brake bias (percentage).
    brake_bias: u8,

    /// Front tyre pressure (PSI).
    front_tyre_pressure: f32,

    /// Rear tyre pressure (PSI).
    rear_tyre_pressure: f32,

    /// Ballast.
    ballast: u8,

    /// Fuel load.
    fuel_load: f32,
}

pub struct CarSetupPacket {
    /// Each packet starts with a packet header.
    pub header: PacketHeader,

    /// The setup for each car in the session. In multiplayer sessions, the
    /// setup for the cars of other players will appear empty.
    pub setups: Vec<CarSetup>,
}
