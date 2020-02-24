use crate::nineteen::{Flag, PacketHeader, VehicleIndex};

pub enum Formula {
    ModernF1 = 0,
    ClassicF1 = 1,
    F2 = 2,
    GenericF1 = 3,
}

pub enum SafetyCar {
    None = 0,
    Full = 1,
    Virtual = 2,
}

pub enum Session {
    Unknown = 0,
    P1 = 1,
    P2 = 2,
    P3 = 3,
    ShortPractice = 4,
    Q1 = 5,
    Q2 = 6,
    Q3 = 7,
    ShortQualifying = 8,
    OneShotQualifying = 9,
    Race = 10,
    Race2 = 11,
    TimeTrial = 12,
}

pub enum Track {
    Unknown = -1,
    Melbourne = 0,
    PaulRicard = 1,
    Shanghai = 2,
    Bahrain = 3,
    Catalunya = 4,
    Monaco = 5,
    Montreal = 6,
    Silverstone = 7,
    Hockenheim = 8,
    Hungaroring = 9,
    Spa = 10,
    Monza = 11,
    Singapore = 12,
    Suzuka = 13,
    AbuDhabi = 14,
    Texas = 15,
    Brazil = 16,
    Austria = 17,
    Sochi = 18,
    Mexico = 19,
    Azerbaijan = 20,
    BahrainShort = 21,
    SilverstoneShort = 22,
    TexasShort = 23,
    SuzukaShort = 24,
}

pub enum Weather {
    Clear = 0,
    LightCloud = 1,
    Overcast = 2,
    LightRain = 3,
    HeavyRain = 4,
    Storm = 5,
}

pub struct MarshalZone {
    /// Fraction of the racetrack's total length, marking the start of the
    /// marshal zone.
    start: f32,

    /// The flag that is currently displayed in the marshal zone.
    flag: Flag,
}

pub struct SessionPacket {
    /// Each packet starts with a packet header.
    pub header: PacketHeader,

    /// Current weather in the session.
    pub weather: Weather,

    /// Track temperature in degrees celsius.
    track_temperature: i8,

    /// Air temperature in degrees celsius.
    air_temperature: i8,

    /// Total number of laps in this race.
    total_laps: u8,

    /// Track length in metres.
    track_length: u16,

    /// Type of the current session.
    session_type: Session,

    /// Current track.
    track: Track,

    /// Current type of formula.
    formula: Formula,

    /// Time left in the session in seconds.
    time_left: u16,

    /// Duration of the session in seconds.
    duration: u16,

    /// Pit speed limit in kilometers per hour
    pit_speed_limit: u8,

    /// Whether the game is paused.
    game_paused: bool,

    /// Whether the player is spectating.
    is_spectating: bool,

    /// Index of the car being spectated.
    spectator_car_index: VehicleIndex,

    /// Whether the support for SLI Pro is active.
    sli_pro_support: bool,

    /// Marshal zones around the track (maximum: 21).
    marshal_zones: Vec<MarshalZone>,

    /// Type of safety car used in the session.
    safety_car: SafetyCar,

    /// Whether session is a multiplayer session.
    network_session: bool,
}
