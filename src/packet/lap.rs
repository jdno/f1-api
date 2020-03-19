//! Data about all the cars in the session and their lap times
//!
//! The F1 games publish data about each car in a session and their lap times. The frequency with
//! which the packets are sent can be configured in the game. F1 2018 and F1 2019 share the same
//! packet format.

use crate::packet::header::Header;
use derive_new::new;
use getset::{CopyGetters, Getters};
use std::time::Duration;

/// Statuses a driver can have during a lap
#[derive(Debug, PartialEq, Copy, Clone, Eq, Ord, PartialOrd, Hash)]
pub enum DriverStatus {
    /// The driver is still in the garage, and has not left it yet.
    InGarage,

    /// The driver is on a flying lap, and cars that are on an in- or out-lap have to give room.
    FlyingLap,

    /// The driver is on an in-lap, i.e. on the way to the pits. The in-lap is often used to cool
    /// the car down, so the driver might be going slower than normal.
    InLap,

    /// The driver is on an out-lap. The out-lap is used to get heat into the tires and breaks to
    /// optimize them for the following flying lap.
    OutLap,

    /// The driver is on track, but not on a special lap. This is the case during a race or practice
    /// session, where drivers do many laps in a row.
    OnTrack,
}

impl Default for DriverStatus {
    fn default() -> Self {
        DriverStatus::InGarage
    }
}

/// Statuses used to signal the progression of a pit stop
#[derive(Debug, PartialEq, Copy, Clone, Eq, Ord, PartialOrd, Hash)]
pub enum PitStatus {
    /// No pit stop is being performed, and the car is most likely on track or in the garage.
    None,

    /// The car is pitting, which means it is on the pit lane but not stationary in the pit box.
    Pitting,

    /// The car is stationary in the pit box, and the pit stop is being performed.
    InPits,
}

impl Default for PitStatus {
    fn default() -> Self {
        PitStatus::None
    }
}

/// Statuses that classify the result
#[derive(Debug, PartialEq, Copy, Clone, Eq, Ord, PartialOrd, Hash)]
pub enum ResultStatus {
    /// The results are invalid.
    Invalid,

    /// The results are not being collected yet.
    Inactive,

    /// The results are being actively collected.
    Active,

    /// The session has finished and the results are final.
    Finished,

    /// The car has been disqualified from the session.
    Disqualified,

    /// The car failed to classify in the session, for example because it did not achieve the
    /// required number of laps.
    NotClassified,

    /// The car has been retired.
    Retired,
}

impl Default for ResultStatus {
    fn default() -> Self {
        ResultStatus::Invalid
    }
}

/// The three sectors of a race track in F1
#[derive(Debug, PartialEq, Copy, Clone, Eq, Ord, PartialOrd, Hash)]
pub enum Sector {
    /// The first sector
    First,

    /// The second sector
    Second,

    /// The third sector
    Third,
}

impl Default for Sector {
    fn default() -> Self {
        Sector::First
    }
}

/// Data about a car and its lap times
///
/// For each car in the session, a set of lap data is published. It contains data on the current
/// lap, e.g. the current lap time and the sector the car is currently in, but also the time of the
/// last and best lap.
#[derive(new, Debug, Getters, CopyGetters, PartialEq, Copy, Clone, PartialOrd, Default)]
#[allow(clippy::too_many_arguments)]
pub struct Lap {
    /// Returns the time of the last lap.
    #[getset(get = "pub")]
    last_lap_time: Duration,

    /// Returns the time of the current lap.
    #[getset(get = "pub")]
    current_lap_time: Duration,

    /// Returns the time of the best lap.
    #[getset(get = "pub")]
    best_lap_time: Duration,

    /// Returns the time spent in sector 1 during the current lap.
    #[getset(get = "pub")]
    sector1_time: Duration,

    /// Returns the time spent in sector 2 during the current lap.
    #[getset(get = "pub")]
    sector2_time: Duration,

    /// Returns the distance the car has travelled in the current lap in meters.
    #[getset(get_copy = "pub")]
    lap_distance: f32,

    /// Returns the total distance the car has travelled in the session in meters.
    #[getset(get_copy = "pub")]
    total_distance: f32,

    /// Returns the delta during a safety car in seconds.
    #[getset(get = "pub")]
    safety_car_delta: Duration,

    /// Returns a car's position in the race.
    #[getset(get_copy = "pub")]
    position: u8,

    /// Returns the number of the current lap.
    #[getset(get_copy = "pub")]
    current_lap_number: u8,

    /// Returns a car's pit stop status.
    #[getset(get_copy = "pub")]
    pit_status: PitStatus,

    /// Returns the sector the car is currently in.
    #[getset(get_copy = "pub")]
    sector: Sector,

    /// Returns whether the current lap is valid.
    ///
    /// The F1 games apply different rules to determine if a lap is valid. Cutting the track, losing
    /// control, or hitting objects or opponents can all invalidate a lap. This is crucial for
    /// qualifying, where invalid laps might not count for the results.
    #[getset(get_copy = "pub")]
    is_lap_valid: bool,

    /// Returns the accumulated penalties for a car in seconds.
    #[getset(get_copy = "pub")]
    penalties: u8,

    /// Returns the grid position the car started the race in.
    #[getset(get_copy = "pub")]
    grid_position: u8,

    /// Returns the status of the driver.
    #[getset(get_copy = "pub")]
    driver_status: DriverStatus,

    /// Returns the status of the race results.
    #[getset(get_copy = "pub")]
    result_status: ResultStatus,
}

/// Packet containing lap data for all 20 cars in a session
///
/// The F1 games publish a lap packet that contains data on all 20 cars in a session. The packet is
/// sent at a fixed interval that can be configured in the game.
#[derive(new, Debug, Getters, PartialEq, Copy, Clone, PartialOrd, Default)]
pub struct LapPacket {
    /// Returns the packet header prefixing the lap data packet.
    #[getset(get = "pub")]
    header: Header,

    /// Returns the laps for all 20 cars in a session.
    #[getset(get = "pub")]
    laps: [Lap; 20],
}
