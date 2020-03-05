use crate::nineteen::{Flag, PacketHeader, VehicleIndex};
use crate::packet::FromBytes;
use bitflags::_core::convert::TryFrom;
use bytes::{Buf, BytesMut};
use std::io::{Cursor, Error, ErrorKind};

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

impl TryFrom<u8> for Formula {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Formula::ModernF1),
            1 => Ok(Formula::ClassicF1),
            2 => Ok(Formula::F2),
            3 => Ok(Formula::GenericF1),
            _ => Err(Error::new(
                ErrorKind::InvalidData,
                "Failed to decode formula.",
            )),
        }
    }
}

impl TryFrom<u8> for SafetyCar {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(SafetyCar::None),
            1 => Ok(SafetyCar::Full),
            2 => Ok(SafetyCar::Virtual),
            _ => Err(Error::new(
                ErrorKind::InvalidData,
                "Failed to decode safety car.",
            )),
        }
    }
}

impl TryFrom<u8> for Session {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Session::Unknown),
            1 => Ok(Session::P1),
            2 => Ok(Session::P2),
            3 => Ok(Session::P3),
            4 => Ok(Session::ShortPractice),
            5 => Ok(Session::Q1),
            6 => Ok(Session::Q2),
            7 => Ok(Session::Q3),
            8 => Ok(Session::ShortQualifying),
            9 => Ok(Session::OneShotQualifying),
            10 => Ok(Session::Race),
            11 => Ok(Session::Race2),
            12 => Ok(Session::TimeTrial),
            _ => Err(Error::new(
                ErrorKind::InvalidData,
                "Failed to decode session.",
            )),
        }
    }
}

impl TryFrom<i8> for Track {
    type Error = Error;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        match value {
            -1 => Ok(Track::Unknown),
            0 => Ok(Track::Melbourne),
            1 => Ok(Track::PaulRicard),
            2 => Ok(Track::Shanghai),
            3 => Ok(Track::Bahrain),
            4 => Ok(Track::Catalunya),
            5 => Ok(Track::Monaco),
            6 => Ok(Track::Montreal),
            7 => Ok(Track::Silverstone),
            8 => Ok(Track::Hockenheim),
            9 => Ok(Track::Hungaroring),
            10 => Ok(Track::Spa),
            11 => Ok(Track::Monza),
            12 => Ok(Track::Singapore),
            13 => Ok(Track::Suzuka),
            14 => Ok(Track::AbuDhabi),
            15 => Ok(Track::Texas),
            16 => Ok(Track::Brazil),
            17 => Ok(Track::Austria),
            18 => Ok(Track::Sochi),
            19 => Ok(Track::Mexico),
            20 => Ok(Track::Azerbaijan),
            21 => Ok(Track::BahrainShort),
            22 => Ok(Track::SilverstoneShort),
            23 => Ok(Track::TexasShort),
            24 => Ok(Track::SuzukaShort),
            _ => Err(Error::new(
                ErrorKind::InvalidData,
                "Failed to decode track.",
            )),
        }
    }
}

impl TryFrom<u8> for Weather {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Weather::Clear),
            1 => Ok(Weather::LightCloud),
            2 => Ok(Weather::Overcast),
            3 => Ok(Weather::LightRain),
            4 => Ok(Weather::HeavyRain),
            5 => Ok(Weather::Storm),
            _ => Err(Error::new(
                ErrorKind::InvalidData,
                "Failed to decode weather.",
            )),
        }
    }
}

impl FromBytes for SessionPacket {
    fn buffer_size() -> usize {
        149
    }

    fn decode(cursor: &mut Cursor<BytesMut>) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let header = PacketHeader::decode(cursor)?;
        let weather = Weather::try_from(cursor.get_u8())?;
        let track_temperature = cursor.get_i8();
        let air_temperature = cursor.get_i8();
        let total_laps = cursor.get_u8();
        let track_length = cursor.get_u16_le();
        let session_type = Session::try_from(cursor.get_u8())?;
        let track = Track::try_from(cursor.get_i8())?;
        let formula = Formula::try_from(cursor.get_u8())?;
        let time_left = cursor.get_u16_le();
        let duration = cursor.get_u16_le();
        let pit_speed_limit = cursor.get_u8();
        let game_paused = cursor.get_u8() > 0;
        let is_spectating = cursor.get_u8() > 0;
        let spectator_car_index = cursor.get_u8();
        let sli_pro_support = cursor.get_u8() > 0;

        let marshal_zone_count = cursor.get_u8();
        let mut marshal_zones = Vec::with_capacity(marshal_zone_count as usize);

        for _ in 0..marshal_zone_count {
            marshal_zones.push(MarshalZone {
                start: cursor.get_f32_le(),
                flag: Flag::try_from(cursor.get_i8())?,
            })
        }

        let safety_car = SafetyCar::try_from(cursor.get_u8())?;
        let network_session = cursor.get_u8() > 0;

        Ok(SessionPacket {
            header,
            weather,
            track_temperature,
            air_temperature,
            total_laps,
            track_length,
            session_type,
            track,
            formula,
            time_left,
            duration,
            pit_speed_limit,
            game_paused,
            is_spectating,
            spectator_car_index,
            sli_pro_support,
            marshal_zones,
            safety_car,
            network_session,
        })
    }
}
