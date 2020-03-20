//! Decoder for session packet sent by F1 2019
//!
//! The session packets by F1 2018 and F1 2019 differ only in their packet headers, the rest of the
//! packet format is identical.

use crate::nineteen::flag::decode_flag;
use crate::nineteen::header::decode_header;
use crate::packet::ensure_packet_size;
use crate::packet::session::{
    Formula, MarshalZone, SafetyCar, Session, SessionPacket, Track, Weather,
};
use bytes::{Buf, BytesMut};
use std::io::{Cursor, Error, ErrorKind};
use std::time::Duration;

/// Size of the session packet in F1 2019
pub const PACKET_SIZE: usize = 149;

/// Decode a session packet sent by F1 2019
///
/// The session packets by F1 2018 and F1 2019 differ only in their packet headers, the rest of the
/// packet format is identical.
pub fn decode_session(cursor: &mut Cursor<&mut BytesMut>) -> Result<SessionPacket, Error> {
    ensure_packet_size(PACKET_SIZE, cursor)?;

    let header = decode_header(cursor)?;

    let weather = decode_weather(cursor)?;
    let track_temperature = cursor.get_i8();
    let air_temperature = cursor.get_i8();
    let total_laps = cursor.get_u8();
    let track_length = cursor.get_u16_le();
    let session_type = decode_session_type(cursor)?;
    let track = decode_track(cursor)?;
    let formula = decode_formula(cursor)?;
    let time_left = Duration::from_secs(cursor.get_u16_le() as u64);
    let duration = Duration::from_secs(cursor.get_u16_le() as u64);
    let pit_speed_limit = cursor.get_u8();
    let game_paused = cursor.get_u8() > 0;
    let is_spectating = cursor.get_u8() > 0;
    let spectator_car_index = cursor.get_u8();
    let sli_pro_support = cursor.get_u8() > 0;

    let marshal_zone_count = cursor.get_u8();
    let mut marshal_zones = Vec::with_capacity(marshal_zone_count as usize);

    for _ in 0..marshal_zone_count {
        marshal_zones.push(MarshalZone::new(cursor.get_f32_le(), decode_flag(cursor)?));
    }

    let safety_car = decode_safety_car(cursor)?;
    let network_session = cursor.get_u8() > 0;

    Ok(SessionPacket::new(
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
    ))
}

fn decode_weather(cursor: &mut Cursor<&mut BytesMut>) -> Result<Weather, Error> {
    let value = cursor.get_u8();

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

fn decode_session_type(cursor: &mut Cursor<&mut BytesMut>) -> Result<Session, Error> {
    let value = cursor.get_u8();

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

fn decode_track(cursor: &mut Cursor<&mut BytesMut>) -> Result<Track, Error> {
    let value = cursor.get_i8();

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

fn decode_formula(cursor: &mut Cursor<&mut BytesMut>) -> Result<Formula, Error> {
    let value = cursor.get_u8();

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

fn decode_safety_car(cursor: &mut Cursor<&mut BytesMut>) -> Result<SafetyCar, Error> {
    let value = cursor.get_u8();

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

#[cfg(test)]
mod tests {
    use crate::nineteen::session::{decode_session, PACKET_SIZE};
    use crate::packet::session::{Formula, SafetyCar, Session, Track, Weather};
    use bytes::{BufMut, BytesMut};
    use std::io::Cursor;

    fn put_packet_header(mut bytes: BytesMut) -> BytesMut {
        bytes.put_u16_le(2019);
        bytes.put_u8(1);
        bytes.put_u8(2);
        bytes.put_u8(3);
        bytes.put_u8(1);
        bytes.put_u64_le(u64::max_value());
        bytes.put_f32_le(1.0);
        bytes.put_u32_le(u32::max_value());
        bytes.put_u8(0);

        bytes
    }

    #[test]
    fn decode_session_with_error() {
        let mut bytes = BytesMut::with_capacity(0);
        let mut cursor = Cursor::new(&mut bytes);

        let packet = decode_session(&mut cursor);
        assert!(packet.is_err());
    }

    #[test]
    fn decode_session_with_success() {
        let mut bytes = BytesMut::with_capacity(PACKET_SIZE);
        bytes = put_packet_header(bytes);

        bytes.put_u8(1);
        bytes.put_i8(2);
        bytes.put_i8(3);
        bytes.put_u8(4);
        bytes.put_u16_le(5);
        bytes.put_u8(6);
        bytes.put_i8(7);
        bytes.put_u8(2);
        bytes.put_u16_le(9);
        bytes.put_u16_le(10);
        bytes.put_u8(11);
        bytes.put_u8(1);
        bytes.put_u8(1);
        bytes.put_u8(14);
        bytes.put_u8(1);
        bytes.put_u8(21);

        for i in 0..21 {
            bytes.put_f32_le(i as f32);
            bytes.put_i8((i % 6) - 1);
        }

        bytes.put_u8(1);
        bytes.put_u8(1);

        let mut cursor = Cursor::new(&mut bytes);

        let packet = decode_session(&mut cursor).unwrap();

        assert_eq!(Weather::LightCloud, packet.weather());
        assert_eq!(2, packet.track_temperature());
        assert_eq!(3, packet.air_temperature());
        assert_eq!(4, packet.total_laps());
        assert_eq!(5, packet.track_length());
        assert_eq!(Session::Q2, packet.session_type());
        assert_eq!(Track::Silverstone, packet.track());
        assert_eq!(Formula::F2, packet.formula());
        assert_eq!(9, packet.time_left().as_secs());
        assert_eq!(10, packet.duration().as_secs());
        assert_eq!(11, packet.pit_speed_limit());
        assert!(packet.game_paused());
        assert!(packet.is_spectating());
        assert_eq!(14, packet.spectator_car_index());
        assert!(packet.sli_pro_support());
        assert_eq!(21, packet.marshal_zones().len());
        assert_eq!(SafetyCar::Full, packet.safety_car());
        assert!(packet.network_session());
    }
}
