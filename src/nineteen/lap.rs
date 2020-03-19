//! Decoder for lap data packets sent by F1 2019
//!
//! The lap data packets by F1 2018 and F1 2019 differ only in their packet headers, the rest of the
//! packet format is identical.

use crate::nineteen::header::decode_header;
use crate::packet::ensure_packet_size;
use crate::packet::lap::{DriverStatus, Lap, LapPacket, PitStatus, ResultStatus, Sector};
use bytes::{Buf, BytesMut};
use std::io::{Cursor, Error, ErrorKind};
use std::time::Duration;

/// Size of the lap data packet in bytes
pub const PACKET_SIZE: usize = 843;

/// Decode a lap data packet sent by F1 2019
///
/// F1 2018 and F1 2019 publish the same data in their lap data packets, but with different packet
/// headers.
pub fn decode_lap_data(cursor: &mut Cursor<&mut BytesMut>) -> Result<LapPacket, Error> {
    ensure_packet_size(PACKET_SIZE, cursor)?;

    let header = decode_header(cursor)?;
    let mut laps = Vec::with_capacity(20);

    for _ in 0..20 {
        laps.push(Lap::new(
            Duration::from_secs_f32(cursor.get_f32_le()),
            Duration::from_secs_f32(cursor.get_f32_le()),
            Duration::from_secs_f32(cursor.get_f32_le()),
            Duration::from_secs_f32(cursor.get_f32_le()),
            Duration::from_secs_f32(cursor.get_f32_le()),
            cursor.get_f32_le(),
            cursor.get_f32_le(),
            Duration::from_secs_f32(cursor.get_f32_le()),
            cursor.get_u8(),
            cursor.get_u8(),
            decode_pit_status(cursor)?,
            decode_sector(cursor)?,
            cursor.get_u8() < 1,
            cursor.get_u8(),
            cursor.get_u8(),
            decode_driver_status(cursor)?,
            decode_result_status(cursor)?,
        ));
    }

    Ok(LapPacket::new(header, laps))
}

fn decode_sector(cursor: &mut Cursor<&mut BytesMut>) -> Result<Sector, Error> {
    let value = cursor.get_u8();

    match value {
        0 => Ok(Sector::First),
        1 => Ok(Sector::Second),
        2 => Ok(Sector::Third),
        _ => Err(Error::new(
            ErrorKind::InvalidData,
            "Failed to decode sector.",
        )),
    }
}

fn decode_pit_status(cursor: &mut Cursor<&mut BytesMut>) -> Result<PitStatus, Error> {
    let value = cursor.get_u8();

    match value {
        0 => Ok(PitStatus::None),
        1 => Ok(PitStatus::Pitting),
        2 => Ok(PitStatus::InPits),
        _ => Err(Error::new(
            ErrorKind::InvalidData,
            "Failed to decode pit status.",
        )),
    }
}

fn decode_driver_status(cursor: &mut Cursor<&mut BytesMut>) -> Result<DriverStatus, Error> {
    let value = cursor.get_u8();

    match value {
        0 => Ok(DriverStatus::InGarage),
        1 => Ok(DriverStatus::FlyingLap),
        2 => Ok(DriverStatus::InLap),
        3 => Ok(DriverStatus::OutLap),
        4 => Ok(DriverStatus::OnTrack),
        _ => Err(Error::new(
            ErrorKind::InvalidData,
            "Failed to decode driver status.",
        )),
    }
}

fn decode_result_status(cursor: &mut Cursor<&mut BytesMut>) -> Result<ResultStatus, Error> {
    let value = cursor.get_u8();

    match value {
        0 => Ok(ResultStatus::Invalid),
        1 => Ok(ResultStatus::Inactive),
        2 => Ok(ResultStatus::Active),
        3 => Ok(ResultStatus::Finished),
        4 => Ok(ResultStatus::Disqualified),
        5 => Ok(ResultStatus::NotClassified),
        6 => Ok(ResultStatus::Retired),
        _ => Err(Error::new(
            ErrorKind::InvalidData,
            "Failed to decode result status.",
        )),
    }
}

#[cfg(test)]
mod tests {
    use crate::nineteen::lap::{decode_lap_data, PACKET_SIZE};
    use crate::packet::lap::{DriverStatus, PitStatus, ResultStatus, Sector};
    use bytes::{BufMut, BytesMut};
    use std::io::Cursor;

    fn put_packet_header(mut bytes: BytesMut) -> BytesMut {
        bytes.put_u16_le(2019);
        bytes.put_u8(1);
        bytes.put_u8(2);
        bytes.put_u8(3);
        bytes.put_u8(0);
        bytes.put_u64_le(u64::max_value());
        bytes.put_f32_le(1.0);
        bytes.put_u32_le(u32::max_value());
        bytes.put_u8(0);

        bytes
    }

    #[test]
    fn decode_lap_data_with_error() {
        let mut bytes = BytesMut::with_capacity(0);
        let mut cursor = Cursor::new(&mut bytes);

        let packet = decode_lap_data(&mut cursor);
        assert!(packet.is_err());
    }

    #[test]
    fn decode_lap_data_with_success() {
        let mut bytes = BytesMut::with_capacity(PACKET_SIZE);
        bytes = put_packet_header(bytes);

        bytes.put_f32_le(62.0);
        bytes.put_f32_le(60.0);
        bytes.put_f32_le(58.1);
        bytes.put_f32_le(21.1);
        bytes.put_f32_le(19.0);
        bytes.put_f32_le(543.0);
        bytes.put_f32_le(2048.4);
        bytes.put_f32_le(0.0);
        bytes.put_u8(1);
        bytes.put_u8(4);
        bytes.put_u8(0);
        bytes.put_u8(2);
        bytes.put_u8(0);
        bytes.put_u8(0);
        bytes.put_u8(3);
        bytes.put_u8(1);
        bytes.put_u8(2);

        let padding = vec![0u8; 779];
        bytes.put(padding.as_slice());

        let mut cursor = Cursor::new(&mut bytes);

        let packet = decode_lap_data(&mut cursor).unwrap();
        let lap = packet.laps()[0];

        assert_eq!(62, lap.last_lap_time().as_secs());
        assert_eq!(60, lap.current_lap_time().as_secs());
        assert_eq!(58, lap.best_lap_time().as_secs());
        assert_eq!(21, lap.sector1_time().as_secs());
        assert_eq!(19, lap.sector2_time().as_secs());
        assert_eq!(543, lap.lap_distance() as usize);
        assert_eq!(2048, lap.total_distance() as usize);
        assert_eq!(0, lap.safety_car_delta().as_secs());
        assert_eq!(1, lap.position());
        assert_eq!(4, lap.current_lap_number());
        assert_eq!(PitStatus::None, lap.pit_status());
        assert_eq!(Sector::Third, lap.sector());
        assert!(lap.is_valid_lap());
        assert_eq!(0, lap.penalties());
        assert_eq!(3, lap.grid_position());
        assert_eq!(DriverStatus::FlyingLap, lap.driver_status());
        assert_eq!(ResultStatus::Active, lap.result_status())
    }
}
