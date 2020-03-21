//! Decoder for telemetry packets sent by F1 2019
//!
//! The telemetry packets by F1 2018 and F1 2019 differ only in their packet headers, the rest of
//! the packet format is identical.

use crate::nineteen::header::decode_header;
use crate::packet::ensure_packet_size;
use crate::packet::telemetry::{Button, Gear, Surface, Telemetry, TelemetryPacket};
use crate::types::CornerProperty;
use bytes::{Buf, BytesMut};
use std::io::{Cursor, Error, ErrorKind};

/// Size of the telemetry packet in bytes
pub const PACKET_SIZE: usize = 1347;

/// Decode a telemetry packet sent by F1 2019
///
/// F1 2018 and F1 2019 publish the same data in their telemetry packets, but with different packet
/// headers.
pub fn decode_telemetry(cursor: &mut Cursor<&mut BytesMut>) -> Result<TelemetryPacket, Error> {
    ensure_packet_size(PACKET_SIZE, cursor)?;

    let header = decode_header(cursor)?;
    let mut telemetry = Vec::with_capacity(20);

    for _ in 0..20 {
        telemetry.push(Telemetry::new(
            cursor.get_u16_le(),
            cursor.get_f32_le(),
            cursor.get_f32_le(),
            cursor.get_f32_le(),
            cursor.get_u8(),
            decode_gear(cursor)?,
            cursor.get_u16_le(),
            cursor.get_u8() > 0,
            cursor.get_u8(),
            decode_brake_temperature(cursor),
            decode_tyre_surface_temperature(cursor),
            decode_tyre_inner_temperature(cursor),
            cursor.get_u16_le(),
            decode_tyre_pressure(cursor),
            decode_surface_type(cursor)?,
        ));
    }

    let button_status = match Button::from_bits(cursor.get_u32_le()) {
        Some(button) => button,
        None => Button::NONE,
    };

    Ok(TelemetryPacket::new(header, telemetry, button_status))
}

fn decode_gear(cursor: &mut Cursor<&mut BytesMut>) -> Result<Gear, Error> {
    let value = cursor.get_i8();

    match value {
        -1 => Ok(Gear::Reverse),
        0 => Ok(Gear::Neutral),
        1 => Ok(Gear::First),
        2 => Ok(Gear::Second),
        3 => Ok(Gear::Third),
        4 => Ok(Gear::Fourth),
        5 => Ok(Gear::Fifth),
        6 => Ok(Gear::Sixth),
        7 => Ok(Gear::Seventh),
        8 => Ok(Gear::Eighth),
        _ => Err(Error::new(ErrorKind::InvalidData, "Failed to decode gear.")),
    }
}

fn decode_brake_temperature(cursor: &mut Cursor<&mut BytesMut>) -> CornerProperty<u16> {
    CornerProperty::new(
        cursor.get_u16_le(),
        cursor.get_u16_le(),
        cursor.get_u16_le(),
        cursor.get_u16_le(),
    )
}

fn decode_tyre_surface_temperature(cursor: &mut Cursor<&mut BytesMut>) -> CornerProperty<u16> {
    CornerProperty::new(
        cursor.get_u16_le(),
        cursor.get_u16_le(),
        cursor.get_u16_le(),
        cursor.get_u16_le(),
    )
}

fn decode_tyre_inner_temperature(cursor: &mut Cursor<&mut BytesMut>) -> CornerProperty<u16> {
    CornerProperty::new(
        cursor.get_u16_le(),
        cursor.get_u16_le(),
        cursor.get_u16_le(),
        cursor.get_u16_le(),
    )
}

fn decode_tyre_pressure(cursor: &mut Cursor<&mut BytesMut>) -> CornerProperty<f32> {
    CornerProperty::new(
        cursor.get_f32_le(),
        cursor.get_f32_le(),
        cursor.get_f32_le(),
        cursor.get_f32_le(),
    )
}

fn decode_surface_type(
    cursor: &mut Cursor<&mut BytesMut>,
) -> Result<CornerProperty<Surface>, Error> {
    Ok(CornerProperty::new(
        decode_surface(cursor)?,
        decode_surface(cursor)?,
        decode_surface(cursor)?,
        decode_surface(cursor)?,
    ))
}

fn decode_surface(cursor: &mut Cursor<&mut BytesMut>) -> Result<Surface, Error> {
    let value = cursor.get_u8();

    match value {
        0 => Ok(Surface::Tarmac),
        1 => Ok(Surface::RumbleStrip),
        2 => Ok(Surface::Concrete),
        3 => Ok(Surface::Rock),
        4 => Ok(Surface::Gravel),
        5 => Ok(Surface::Mud),
        6 => Ok(Surface::Sand),
        7 => Ok(Surface::Grass),
        8 => Ok(Surface::Water),
        9 => Ok(Surface::Cobblestone),
        10 => Ok(Surface::Metal),
        11 => Ok(Surface::Ridged),
        _ => Err(Error::new(
            ErrorKind::InvalidData,
            "Failed to decode surface.",
        )),
    }
}

#[cfg(test)]
mod tests {
    use crate::nineteen::telemetry::{decode_telemetry, PACKET_SIZE};
    use crate::packet::telemetry::{Button, Gear, Surface};
    use assert_approx_eq::assert_approx_eq;
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
    fn decode_telemetry_with_error() {
        let mut bytes = BytesMut::with_capacity(0);
        let mut cursor = Cursor::new(&mut bytes);

        let packet = decode_telemetry(&mut cursor);
        assert!(packet.is_err());
    }

    #[test]
    fn decode_telemetry_with_success() {
        let mut bytes = BytesMut::with_capacity(PACKET_SIZE);
        bytes = put_packet_header(bytes);

        for _ in 0..20 {
            bytes.put_u16_le(1);
            bytes.put_f32_le(2.0);
            bytes.put_f32_le(3.0);
            bytes.put_f32_le(4.0);
            bytes.put_u8(5);
            bytes.put_u8(6);
            bytes.put_u16_le(7);
            bytes.put_u8(1);
            bytes.put_u8(9);
            bytes.put_u16_le(10);
            bytes.put_u16_le(11);
            bytes.put_u16_le(12);
            bytes.put_u16_le(13);
            bytes.put_u16_le(14);
            bytes.put_u16_le(15);
            bytes.put_u16_le(16);
            bytes.put_u16_le(17);
            bytes.put_u16_le(18);
            bytes.put_u16_le(19);
            bytes.put_u16_le(20);
            bytes.put_u16_le(21);
            bytes.put_u16_le(22);
            bytes.put_f32_le(23.0);
            bytes.put_f32_le(24.0);
            bytes.put_f32_le(25.0);
            bytes.put_f32_le(26.0);
            bytes.put_u8(5);
            bytes.put_u8(6);
            bytes.put_u8(7);
            bytes.put_u8(8);
        }

        bytes.put_u32_le(0x0001);

        let mut cursor = Cursor::new(&mut bytes);

        let packet = decode_telemetry(&mut cursor).unwrap();
        let telemetry = packet.telemetry()[0];

        assert_eq!(1, telemetry.speed());
        assert_approx_eq!(2.0, telemetry.throttle());
        assert_approx_eq!(3.0, telemetry.steering());
        assert_approx_eq!(4.0, telemetry.brake());
        assert_eq!(5, telemetry.clutch());
        assert_eq!(Gear::Sixth, telemetry.gear());
        assert_eq!(7, telemetry.engine_rpm());
        assert!(telemetry.drs());
        assert_eq!(9, telemetry.rev_lights());
        assert_eq!(10, telemetry.brake_temperature().front_left());
        assert_eq!(14, telemetry.tyre_surface_temperature().front_left());
        assert_eq!(18, telemetry.tyre_inner_temperature().front_left());
        assert_eq!(22, telemetry.engine_temperature());
        assert_approx_eq!(23.0, telemetry.tyre_pressure().front_left());
        assert_eq!(Surface::Mud, telemetry.surface_type().front_left());
        assert_eq!(Button::CROSS_OR_A, packet.button_status());
    }
}
