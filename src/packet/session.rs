//! Data about the current session
//!
//! The F1 games provide information about the current session, for example weather and temperature
//! as well as settings like the type of safety car in use.

use std::time::Duration;

use derive_new::new;
use getset::{CopyGetters, Getters};

use crate::packet::header::Header;
use crate::types::{Flag, VehicleIndex};

/// Types of formula racing supported by the F1 games
///
/// The F1 games support different types of formula racing, with newer games typically supporting
/// more than older games.
#[derive(Debug, PartialEq, Copy, Clone, Eq, Ord, PartialOrd, Hash)]
pub enum Formula {
    ClassicF1,
    GenericF1,
    ModernF1,
    F2,
}

impl Default for Formula {
    fn default() -> Self {
        Formula::ModernF1
    }
}

/// Safety car rules that can be set for a session
///
/// The F1 games allow different rules to be configured for the safety car. Sessions can have no
/// safety car at all, a virtual safety car, or a full safety car.
#[derive(Debug, PartialEq, Copy, Clone, Eq, Ord, PartialOrd, Hash)]
pub enum SafetyCar {
    None,
    Full,
    Virtual,
}

impl Default for SafetyCar {
    fn default() -> Self {
        SafetyCar::Full
    }
}

/// Types of sessions
///
/// F1 knows many different types of sessions. A typical race weekend consists of free practice,
/// qualifying and a race, each of which can be divided into multiple sessions (e.g. first or second
/// free practice).
#[derive(Debug, PartialEq, Copy, Clone, Eq, Ord, PartialOrd, Hash)]
pub enum Session {
    OneShotQualifying,
    P1,
    P2,
    P3,
    Q1,
    Q2,
    Q3,
    Race,
    Race2,
    ShortPractice,
    ShortQualifying,
    TimeTrial,
    Unknown,
}

impl Default for Session {
    fn default() -> Self {
        Session::Unknown
    }
}

/// Race tracks that are in the F1 games
///
/// The F1 games feature a long list of race tracks that appear in the games. Not every track is
/// available in every game.
#[derive(Debug, PartialEq, Copy, Clone, Eq, Ord, PartialOrd, Hash)]
pub enum Track {
    AbuDhabi,
    Austria,
    Azerbaijan,
    Bahrain,
    BahrainShort,
    Brazil,
    Catalunya,
    Hockenheim,
    Hungaroring,
    Melbourne,
    Mexico,
    Monaco,
    Montreal,
    Monza,
    PaulRicard,
    Shanghai,
    Silverstone,
    SilverstoneShort,
    Singapore,
    Sochi,
    Spa,
    Suzuka,
    SuzukaShort,
    Texas,
    TexasShort,
    Unknown,
}

impl Default for Track {
    fn default() -> Self {
        Track::Unknown
    }
}

/// Weather conditions that can occur in a session
///
/// The modern F1 games support changing weather conditions, though not every weather condition is
/// supported by every game.
#[derive(Debug, PartialEq, Copy, Clone, Eq, Ord, PartialOrd, Hash)]
pub enum Weather {
    Clear,
    LightCloud,
    Overcast,
    LightRain,
    HeavyRain,
    Storm,
}

impl Default for Weather {
    fn default() -> Self {
        Weather::Clear
    }
}

/// A marshal zone around the track and its current flags.
///
/// A race track is divided into many marshal zones. In each zone, flags can be waved to inform
/// drivers about hazards on track, faster cars approaching from behind, and other important status
/// updates. Each zone is represented by a struct containing the fraction of the race track's length
/// where the zone starts, and any flag that is currently being shown there.
#[derive(new, Debug, CopyGetters, PartialEq, Copy, Clone, PartialOrd, Default)]
pub struct MarshalZone {
    /// Returns the start point of the marshal zone as a fraction of the race track's total length.
    #[getset(get_copy = "pub")]
    start: f32,

    /// Returns the flag that is currently being displayed in the marshal zone.
    #[getset(get_copy = "pub")]
    flag: Flag,
}

/// Packet containing data about the current session
///
/// The session packet provides information about the current session, for example weather and
/// temperature as well as settings like the type of safety car in use.
#[derive(new, Debug, CopyGetters, Getters, PartialEq, Clone, PartialOrd)]
#[allow(clippy::too_many_arguments)]
pub struct SessionPacket {
    /// Returns the packet header prefixing the session packet.
    #[getset(get = "pub")]
    header: Header,

    /// Returns the current weather in the session.
    #[getset(get_copy = "pub")]
    weather: Weather,

    /// Returns the track temperature in degrees celsius.
    #[getset(get_copy = "pub")]
    track_temperature: i8,

    /// Returns the air temperature in degrees celsius.
    #[getset(get_copy = "pub")]
    air_temperature: i8,

    /// Returns the total number of laps in this race.
    #[getset(get_copy = "pub")]
    total_laps: u8,

    /// Returns the length of the race track in metres.
    #[getset(get_copy = "pub")]
    track_length: u16,

    /// Returns the type of the current session.
    #[getset(get_copy = "pub")]
    session_type: Session,

    /// Returns the race track of the session.
    #[getset(get_copy = "pub")]
    track: Track,

    /// Returns the type of formula racing.
    #[getset(get_copy = "pub")]
    formula: Formula,

    /// Returns the time that is left in the session.
    #[getset(get = "pub")]
    time_left: Duration,

    /// Returns the duration of the session in seconds.
    #[getset(get = "pub")]
    duration: Duration,

    /// Returns the pit speed limit in kilometers per hour.
    #[getset(get_copy = "pub")]
    pit_speed_limit: u8,

    /// Returns whether the game is paused right now.
    #[getset(get_copy = "pub")]
    game_paused: bool,

    /// Returns whether the player is spectating the session.
    #[getset(get_copy = "pub")]
    is_spectating: bool,

    /// Returns the index of the car being spectated.
    #[getset(get_copy = "pub")]
    spectator_car_index: VehicleIndex,

    /// Returns whether the support for SLI Pro is active.
    #[getset(get_copy = "pub")]
    sli_pro_support: bool,

    /// Returns the marshal zones around the track.
    #[getset(get = "pub")]
    marshal_zones: Vec<MarshalZone>,

    /// Returns the type of safety car that is used in the session.
    #[getset(get_copy = "pub")]
    safety_car: SafetyCar,

    /// Returns whether the session is a multiplayer session.
    #[getset(get_copy = "pub")]
    network_session: bool,
}
