//! API specification for F1 2019.

pub mod event;
pub mod lap;
pub mod motion;
pub mod participants;
pub mod session;
pub mod setup;
pub mod status;
pub mod telemetry;

pub enum Flag {
    Invalid = -1,
    None = 0,
    Green = 1,
    Blue = 2,
    Yellow = 3,
    Red = 4,
}

pub struct PacketHeader {
    /// The packet format is the version of the packet. Newer games can fall
    /// back to older packet formats to ensure interoperability with existing
    /// tools. Usually, this is the year of the release, e.g. `2018` or `2019`.
    pub packet_format: u16,

    /// The game is versioned using the format `MAJOR.MINOR`. This field
    /// contains the game's major version.
    pub game_major_version: u8,

    /// The game is versioned using the format `MAJOR.MINOR`. This field
    /// contains the game's minor version.
    pub game_minor_version: u8,

    /// The F1 games send different packets containing different data. Each type
    /// of packet can be versioned as well to allow for API changes. The version
    /// of a packet starts at `1` and increments.
    pub packet_version: u8,

    /// The F1 games send different packets containing different data. Each type
    /// of packet has a unique Id that is used to identify, and the properly
    /// parse, the packet.
    pub packet_id: u8,

    /// Each session is identified by a unique identifer.
    pub session_uid: u64,

    /// Each packet contains a timestamp, marking the time the data was
    /// captured.
    pub session_time: f32,

    /// Each packet contains an identifier for the frame the data was retrieved
    /// on.
    pub frame_identifier: u32,

    /// The setups and status of cars are published as arrays. This field
    /// indicates which position in these arrays the player's car has.
    pub player_car_index: u8,
}
