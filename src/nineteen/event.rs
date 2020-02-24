use crate::nineteen::PacketHeader;

pub struct FastestLap {
    /// The index of the car achieving the fastest lap.
    vehicle_index: u8,

    /// The lap time in seconds.
    lap_time: f32,
}

pub struct Retirement {
    /// The index of the car that is retiring from the race.
    vehicle_index: u8,
}

pub struct TeammateInPits {
    /// The index of the teammate's car, who is currently pitting.
    vehicle_index: u8,
}

pub struct RaceWinner {
    /// The index of the car winning the race.
    vehicle_index: u8,
}

pub enum Event {
    SessionStarted,
    SessionEnded,
    FastestLap(FastestLap),
    Retirement(Retirement),
    DrsEnabled,
    DrsDisabled,
    TeammatesInPits(TeammateInPits),
    ChequeredFlag,
    RaceWinner(RaceWinner),
}

pub struct EventPacket {
    /// Each packet starts with a packet header.
    pub header: PacketHeader,

    /// The type of event.
    pub event: Event,
}
