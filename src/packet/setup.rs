//! Data about car setups
//!
//! The F1 games publish data about the setups of all cars in a session. In multiplayer sessions,
//! setups of other players are redacted to prevent anyone from gaining an unfair advantage.

use crate::packet::header::Header;
use derive_new::new;
use getset::{CopyGetters, Getters};

/// Setup of a car
///
/// The setup of a car in the F1 games consists of a set of parameters that players can adjust
/// before leaving the garage.
#[derive(new, Debug, CopyGetters, Getters, PartialEq, Copy, Clone, PartialOrd, Default)]
#[allow(clippy::too_many_arguments)]
pub struct CarSetup {
    /// Returns the setting for the front wing aero.
    #[getset(get_copy = "pub")]
    front_wing: u8,

    /// Returns the setting for the rear wing aero.
    #[getset(get_copy = "pub")]
    rear_wing: u8,

    /// Returns the differential adjustment on throttle as a percentage.
    #[getset(get_copy = "pub")]
    on_throttle: u8,

    /// Returns the differential adjustment off throttle as a percentage.
    #[getset(get_copy = "pub")]
    off_throttle: u8,

    /// Returns the setting for the front camber angle.
    #[getset(get_copy = "pub")]
    front_camber: f32,

    /// Returns the setting for the rear camber angle.
    #[getset(get_copy = "pub")]
    rear_camber: f32,

    /// Returns the setting for the front toe angle.
    #[getset(get_copy = "pub")]
    front_toe: f32,

    /// Returns the setting for the rear toe angle.
    #[getset(get_copy = "pub")]
    rear_toe: f32,

    /// Returns the front suspension setting.
    #[getset(get_copy = "pub")]
    front_suspension: u8,

    /// Returns the rear suspension setting.
    #[getset(get_copy = "pub")]
    rear_suspension: u8,

    /// Returns the setting for the front anti-roll bar.
    #[getset(get_copy = "pub")]
    front_anti_roll_bar: u8,

    /// Returns the setting for the rear anti-roll bar.
    #[getset(get_copy = "pub")]
    rear_anti_roll_bar: u8,

    /// Returns the setting for the front ride height.
    #[getset(get_copy = "pub")]
    front_suspension_height: u8,

    /// Returns the setting for the rear right height.
    #[getset(get_copy = "pub")]
    rear_suspension_height: u8,

    /// Returns the setting for the brake pressure as a percentage.
    #[getset(get_copy = "pub")]
    brake_pressure: u8,

    /// Returns the setting for the brake bias as a percentage.
    #[getset(get_copy = "pub")]
    brake_bias: u8,

    /// Returns the setting for the front tyre pressure in psi.
    #[getset(get_copy = "pub")]
    front_tyre_pressure: f32,

    /// Returns the setting for the rear tyre pressure in psi.
    #[getset(get_copy = "pub")]
    rear_tyre_pressure: f32,

    /// Returns the setting for additional ballast.
    #[getset(get_copy = "pub")]
    ballast: u8,

    /// Returns the setting for the fuel load.
    #[getset(get_copy = "pub")]
    fuel_load: f32,
}

/// Packet containing the setups of all cars in the session
///
/// The F1 games publish the setup of each car in the session in the car setup packet. In
/// multiplayer sessions, the setups of other players are redacted to prevent anyone from gaining an
/// unfair advantage.
#[derive(new, Debug, Getters, PartialEq, Clone, PartialOrd, Default)]
pub struct CarSetupPacket {
    /// Returns the packet header prefixing the car setup packet.
    #[getset(get = "pub")]
    header: Header,

    /// Returns the setups of all 20 cars in the session.
    #[getset(get = "pub")]
    setups: Vec<CarSetup>,
}
