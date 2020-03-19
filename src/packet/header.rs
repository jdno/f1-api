//! Header prefixing packets from modern F1 games

use crate::packet::VehicleIndex;
use derive_new::new;
use getset::{CopyGetters, Getters};
use std::fmt;
use std::fmt::Display;
use std::time::Duration;

/// Version number of the game
///
/// The modern F1 games include their version number in the packet header. The games are versioned
/// using the scheme `MAJOR.MINOR`.
///
/// TODO Test that partial order works correctly with version numbers
#[derive(
    new, Debug, Getters, CopyGetters, PartialEq, Copy, Clone, Eq, Ord, PartialOrd, Hash, Default,
)]
pub struct GameVersion {
    /// Returns the major version of the game.
    #[getset(get_copy = "pub")]
    major: u8,

    /// Returns the minor version of the game.
    #[getset(get_copy = "pub")]
    minor: u8,
}

impl Display for GameVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}", self.major, self.minor)
    }
}

/// Header prefixing each packet
///
/// The modern F1 games use versioned API specifications. Each packet is prefixed with a header that
/// declares which version of the specification the packet adheres to. This information is required
/// to decode the packet correctly. Because it is only relevant for decoding the packet, the packet
/// format, type, and version from the specifications are not republished.
///
/// The header also contains information about the session the packet belongs to, and about the time
/// the packet was created.
///
/// TODO Verify that the session tie can be represented as a duration
#[derive(
    new, Debug, Getters, CopyGetters, PartialEq, Copy, Clone, Eq, Ord, PartialOrd, Hash, Default,
)]
pub struct Header {
    /// Returns the version of the game.
    #[getset(get = "pub")]
    game_version: Option<GameVersion>,

    /// Returns the unique session UID.
    #[getset(get_copy = "pub")]
    session_uid: u64,

    /// Returns the session time at the time the packet was sent.
    #[getset(get = "pub")]
    session_time: Duration,

    /// Returns the frame identifier at the time the packet was sent.
    #[getset(get_copy = "pub")]
    frame_identifier: u32,

    /// Returns the player's car index.
    ///
    /// The setups and status of cars are published as arrays. This field indicates which position
    /// in these arrays the player's car has.
    #[getset(get_copy = "pub")]
    player_car_index: VehicleIndex,
}

impl Display for Header {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let game_version = match self.game_version {
            Some(version) => format!("{}", version),
            None => String::from("None"),
        };

        write!(
            f,
            "Header {{ game_version: {}, session: {}, time: {}s, frame: {}, player_car_index: {} }}",
            game_version,
            self.session_uid,
            self.session_time.as_secs(),
            self.frame_identifier,
            self.player_car_index
        )
    }
}
