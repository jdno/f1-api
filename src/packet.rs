//! Packet definitions

use crate::nineteen;

pub mod event;
pub mod header;

/// A packet published by an F1 game.
///
/// The F1 games publish different packets with different data at different intervals. Each of these
/// packets is decoded from UDP to their respective representation in this Rust crate. The `Packet`
/// enum lists all packets that can be expected, and that a client should handle.
pub enum Packet {
    /// Packet from F1 2019
    Nineteen(nineteen::Packet),
}

/// Reference to a vehicle in a packet
///
/// In Formula 1, a maximum of 20 cars can participate in any session. The modern F1 games use this
/// rule to use arrays with a static size of 20 whenever they publish data about all vehicles in a
/// session. Data in those arrays is referenced using an unsigned byte. By defining a type alias for
/// the indices, their usage can be checked by the Rust compiler.
pub type VehicleIndex = u8;
