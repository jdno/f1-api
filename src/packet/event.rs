//! Events that can occur during the course of a session
//!
//! The F1 games send event packets whenever certain events occur in a session. _F1 2018_ defined
//! only two events, but _F1 2019_ extended this to nine different events. Some events carry a
//! payload that further defines the event, and that are declared in this module as structs.

use crate::packet::header::Header;
use crate::packet::VehicleIndex;
use getset::{CopyGetters, Getters};
use std::fmt;
use std::fmt::Display;
use std::time::Duration;

/// Payload for fastest lap event
///
/// The fastest lap event contains the driver achieving the fastest lap as well as the lap time as
/// its payload. The driver is referenced through the vehicle index, while the lap time is provided
/// in seconds.
///
/// # Examples
///
/// ```
/// # use f1_api::packet::event::{FastestLap, Event};
/// # use std::time::Duration;
/// #
/// # let fastest_lap = FastestLap::new(0, Duration::from_secs(62));
/// # let event = Event::FastestLap(fastest_lap);
/// #
/// // Simplified use in a match statement
/// match event {
///     Event::FastestLap(lap) => {
///         assert_eq!(0, lap.vehicle_index());
///         assert_eq!(62, lap.time().as_secs());
///     }
/// #   _ => panic!("Example should never fail")
/// }
/// ```
#[derive(
    Debug, Getters, CopyGetters, PartialEq, Copy, Clone, Eq, Ord, PartialOrd, Hash, Default,
)]
pub struct FastestLap {
    /// Returns the index of the car achieving the fastest lap.
    #[getset(get_copy = "pub")]
    vehicle_index: VehicleIndex,

    /// Returns the time of the fastest lap.
    #[getset(get = "pub")]
    time: Duration,
}

impl FastestLap {
    /// Returns a new instance of the fastest lap payload.
    pub fn new(vehicle_index: VehicleIndex, time: Duration) -> Self {
        FastestLap {
            vehicle_index,
            time,
        }
    }
}

impl Display for FastestLap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}s by car #{}",
            self.time.as_secs_f32(),
            self.vehicle_index
        )
    }
}

/// Payload for retirement event
///
/// The retirement event contains the vehicle index of the retired driver as its payload.
///
/// # Examples
///
/// ```
/// # use f1_api::packet::event::{Event, Retirement};
/// #
/// # let retirement = Retirement::new(0);
/// # let event = Event::Retirement(retirement);
/// #
/// // Simplified use in a match statement
/// match event {
///     Event::Retirement(retirement) => {
///         assert_eq!(0, retirement.vehicle_index());
///     }
/// #   _ => panic!("Example should never fail")
/// }
/// ```
#[derive(
    Debug, Getters, CopyGetters, PartialEq, Copy, Clone, Eq, Ord, PartialOrd, Hash, Default,
)]
pub struct Retirement {
    /// Returns the index of the car retiring.
    #[getset(get_copy = "pub")]
    vehicle_index: VehicleIndex,
}

impl Retirement {
    /// Returns a new instance of a retirement payload.
    pub fn new(vehicle_index: VehicleIndex) -> Self {
        Retirement { vehicle_index }
    }
}

impl Display for Retirement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "retirement of car #{}", self.vehicle_index)
    }
}

/// Payload for teammate in pits event
///
/// When a teammate enters the pits, an event is sent carrying the vehicle index of the teammate as
/// its payload.
///
/// # Examples
///
/// ```
/// # use f1_api::packet::event::{Event, TeammateInPits};
/// #
/// # let teammate_in_pits = TeammateInPits::new(0);
/// # let event = Event::TeammatesInPits(teammate_in_pits);
/// #
/// // Simplified use in a match statement
/// match event {
///     Event::TeammatesInPits(teammate) => {
///         assert_eq!(0, teammate.vehicle_index());
///     }
/// #   _ => panic!("Example should never fail")
/// }
/// ```
#[derive(
    Debug, Getters, CopyGetters, PartialEq, Copy, Clone, Eq, Ord, PartialOrd, Hash, Default,
)]
pub struct TeammateInPits {
    /// Returns the index of the teammate who has just entered the pits.
    #[getset(get_copy = "pub")]
    vehicle_index: VehicleIndex,
}

impl TeammateInPits {
    /// Returns a new instance of a teammates in the pits payload.
    pub fn new(vehicle_index: VehicleIndex) -> Self {
        TeammateInPits { vehicle_index }
    }
}

impl Display for TeammateInPits {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "teammate in car #{} in pits", self.vehicle_index)
    }
}

/// Payload for the race winner event
///
/// The event announcing the race winner carries the vehicle index of the winner as its payload.
///
/// # Examples
///
/// ```
/// # use f1_api::packet::event::{Event, RaceWinner};
/// #
/// # let race_winner = RaceWinner::new(0);
/// # let event = Event::RaceWinner(race_winner);
/// #
/// // Simplified use in a match statement
/// match event {
///     Event::RaceWinner(winner) => {
///         assert_eq!(0, winner.vehicle_index());
///     }
/// #   _ => panic!("Example should never fail")
/// }
/// ```
#[derive(
    Debug, Getters, CopyGetters, PartialEq, Copy, Clone, Eq, Ord, PartialOrd, Hash, Default,
)]
pub struct RaceWinner {
    /// Returns the index of the car that has won the race.
    #[getset(get_copy = "pub")]
    vehicle_index: VehicleIndex,
}

impl RaceWinner {
    /// Returns a new instance of the race winner payload.
    pub fn new(vehicle_index: VehicleIndex) -> Self {
        RaceWinner { vehicle_index }
    }
}

impl Display for RaceWinner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "race winner in car #{}", self.vehicle_index)
    }
}

/// Events that can occur during the course of a session
///
/// The F1 games send event packets whenever a certain event occurs in a session. Depending on the
/// game, only a subset of the defined events may be published. Some events carry a payload that
/// further describes the event. For example, the event declaring the race winner sends with it the
/// vehicle index of said winner.
#[derive(Debug, PartialEq, Copy, Clone, Eq, Ord, PartialOrd, Hash)]
pub enum Event {
    /// The chequered flag signals the end of the race.
    ChequeredFlag,

    /// DRS is disabled at the beginning of the race, and can be disabled throughout the race in
    /// case of poor weather conditions or yellow flags in the DRS activation zone.
    DrsDisabled,

    /// DRS gets enabled after the first two laps of a race. In case DRS is disabled during a race,
    /// e.g. due to poor weather conditions, it can be re-enabled once the conditions have cleared.
    DrsEnabled,

    /// When a driver achieves the fastest lap of the race, the event publishes the driver and their
    /// time.
    FastestLap(FastestLap),

    /// At the end of the race, the race winner is announced in an event.
    RaceWinner(RaceWinner),

    /// Drivers can retire from a race, for example after their car suffers technical issues. The
    /// retirement is announced as an event with the driver as the payload.
    Retirement(Retirement),

    /// The end of a session is announced in an event.
    SessionEnded,

    /// The start of a session is announced in an event.
    SessionStarted,

    /// When a teammate enters the pits, an event carrying their vehicle index is published.
    TeammatesInPits(TeammateInPits),
}

impl Default for Event {
    fn default() -> Self {
        Event::SessionStarted
    }
}

impl Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Event::SessionStarted => write!(f, "Session started"),
            Event::SessionEnded => write!(f, "Session ended"),
            Event::FastestLap(lap) => write!(
                f,
                "Fastest lap by car #{} ({}s)",
                lap.vehicle_index,
                lap.time.as_secs_f32()
            ),
            Event::Retirement(retirement) => write!(f, "Car #{} retired", retirement.vehicle_index),
            Event::DrsEnabled => write!(f, "DRS enabled"),
            Event::DrsDisabled => write!(f, "DRS disabled"),
            Event::TeammatesInPits(teammate) => {
                write!(f, "Teammate in car #{} in pits", teammate.vehicle_index)
            }
            Event::ChequeredFlag => write!(f, "Chequered flag"),
            Event::RaceWinner(winner) => write!(f, "Car #{} won the race", winner.vehicle_index),
        }
    }
}

/// Packet containing details about an event that occurred in the session
///
/// The modern F1 games send event packets with details about events that occur in a session. The
/// frequency with which these packets are sent is not fixed, but rather packets are sent whenever
/// events occur.
#[derive(Debug, Getters, PartialEq, Copy, Clone, Eq, Ord, PartialOrd, Hash, Default)]
pub struct EventPacket {
    /// Returns the packet header prefixing the event packet.
    #[getset(get = "pub")]
    header: Header,

    /// Returns the event from the event packet.
    #[getset(get = "pub")]
    event: Event,
}

impl EventPacket {
    /// Returns a new instance of an event packet.
    pub fn new(header: Header, event: Event) -> Self {
        EventPacket { header, event }
    }
}

impl Display for EventPacket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "EventPacket {{ header: {}, event: {} }}",
            self.header, self.event
        )
    }
}
