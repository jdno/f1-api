//! Packet definitions

use crate::nineteen;

/// A packet published by an F1 game.
///
/// The F1 games publish different packets with different data at different intervals. Each of these
/// packets is decoded from UDP to their respective representation in this Rust crate. The `Packet`
/// enum lists all packets that can be expected, and that a client should handle.
pub enum Packet {
    /// Packet from F1 2019
    Nineteen(nineteen::Packet),
}
