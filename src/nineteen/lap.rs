use crate::nineteen::PacketHeader;

pub enum DriverStatus {
    InGarage = 0,
    FlyingLap = 1,
    InLap = 2,
    OutLap = 3,
    OnTrack = 4,
}

pub enum PitStatus {
    None = 0,
    Pitting = 1,
    InPits = 2,
}

pub enum ResultStatus {
    Invalid = 0,
    Inactive = 1,
    Active = 2,
    Finished = 3,
    Disqualified = 4,
    NotClassified = 5,
    Retired = 6,
}

pub enum Sector {
    First = 0,
    Second = 1,
    Third = 2,
}

pub struct Lap {
    /// Last lap time in seconds.
    last_lap_time: f32,

    /// Current time around the lap in seconds.
    curent_lap_time: f32,

    /// Best lap time in the session in seconds.
    best_lap_time: f32,

    /// Time in sector 1 in seconds.
    sector1_time: f32,

    /// Time in sector 2 in seconds.
    sector2_time: f32,

    /// Distance the vehicle is around the current lap in meters. Can be
    /// negative if the car has not crossed the line yet (e.g. in qualifying).
    lap_distance: f32,

    /// Total distance traveled in session in meters. Can be negative if the car
    /// has not crossed the line yet (e.g. in qualifying).
    total_distance: f32,

    /// Delta in seconds for safety car.
    safety_car_delta: f32,

    /// Position of the car in the race.
    position: u8,

    /// Number of the current lap.
    current_lap_number: u8,

    /// Pit status.
    pit_status: PitStatus,

    /// Current sector.
    sector: Sector,

    /// Whether the current lap is valid.
    is_lap_valid: bool,

    /// Accumulated time penalties to be added in seconds.
    penalties: u8,

    /// Grid position the vehicle started the race in.
    grid_position: u8,

    /// Driver status.
    driver_status: DriverStatus,

    /// Result status.
    result_status: ResultStatus,
}

pub struct LapPacket {
    /// Each packet starts with a packet header.
    pub header: PacketHeader,

    /// Lap data for all cars on track.
    pub laps: Vec<Lap>,
}
