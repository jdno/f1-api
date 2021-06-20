//! Decoder for motion packets sent by F1 2019
//!
//! The motion packets by F1 2018 and F1 2019 differ only in their packet headers, the rest of the
//! packet format is identical.

use std::io::{Cursor, Error};

use bytes::{Buf, BytesMut};

use crate::nineteen::header::decode_header;
use crate::packet::ensure_packet_size;
use crate::packet::motion::{Motion, MotionPacket};
use crate::types::{CornerProperty, Property3D};

/// Size of the motion packet in bytes
pub const PACKET_SIZE: usize = 1343;

/// Decode a motion packet sent by F1 2019
///
/// F1 2018 and F1 2019 publish the same data in their motion packets, but with different packet
/// headers.
pub fn decode_motion(cursor: &mut Cursor<&mut BytesMut>) -> Result<MotionPacket, Error> {
    ensure_packet_size(PACKET_SIZE, cursor)?;

    let header = decode_header(cursor)?;
    let mut cars = Vec::with_capacity(20);

    for _ in 0..20 {
        cars.push(Motion::new(
            decode_position(cursor),
            decode_velocity(cursor),
            decode_forward_direction(cursor),
            decode_right_direction(cursor),
            decode_g_force(cursor),
            cursor.get_f32_le(),
            cursor.get_f32_le(),
            cursor.get_f32_le(),
        ))
    }

    Ok(MotionPacket::new(
        header,
        cars,
        decode_suspension_position(cursor),
        decode_suspension_velocity(cursor),
        decode_suspension_acceleration(cursor),
        decode_wheel_speed(cursor),
        decode_wheel_slip(cursor),
        decode_local_velocity(cursor),
        decode_angular_velocity(cursor),
        decode_angular_acceleration(cursor),
        cursor.get_f32_le(),
    ))
}

/// Decode position of the car
fn decode_position(cursor: &mut Cursor<&mut BytesMut>) -> Property3D<f32> {
    Property3D::new(
        cursor.get_f32_le(),
        cursor.get_f32_le(),
        cursor.get_f32_le(),
    )
}

/// Decode velocity of the car
fn decode_velocity(cursor: &mut Cursor<&mut BytesMut>) -> Property3D<f32> {
    Property3D::new(
        cursor.get_f32_le(),
        cursor.get_f32_le(),
        cursor.get_f32_le(),
    )
}

/// Decode forward direction of the car
fn decode_forward_direction(cursor: &mut Cursor<&mut BytesMut>) -> Property3D<i16> {
    Property3D::new(
        cursor.get_i16_le(),
        cursor.get_i16_le(),
        cursor.get_i16_le(),
    )
}

/// Decode right direction of the car
fn decode_right_direction(cursor: &mut Cursor<&mut BytesMut>) -> Property3D<i16> {
    Property3D::new(
        cursor.get_i16_le(),
        cursor.get_i16_le(),
        cursor.get_i16_le(),
    )
}

/// Decode G forces on the car
fn decode_g_force(cursor: &mut Cursor<&mut BytesMut>) -> Property3D<f32> {
    Property3D::new(
        cursor.get_f32_le(),
        cursor.get_f32_le(),
        cursor.get_f32_le(),
    )
}

/// Decode suspension position of the player's car
fn decode_suspension_position(cursor: &mut Cursor<&mut BytesMut>) -> CornerProperty<f32> {
    CornerProperty::new(
        cursor.get_f32_le(),
        cursor.get_f32_le(),
        cursor.get_f32_le(),
        cursor.get_f32_le(),
    )
}

/// Decode suspension velocity of the player's car
fn decode_suspension_velocity(cursor: &mut Cursor<&mut BytesMut>) -> CornerProperty<f32> {
    CornerProperty::new(
        cursor.get_f32_le(),
        cursor.get_f32_le(),
        cursor.get_f32_le(),
        cursor.get_f32_le(),
    )
}

/// Decode suspension acceleration of the player's car
fn decode_suspension_acceleration(cursor: &mut Cursor<&mut BytesMut>) -> CornerProperty<f32> {
    CornerProperty::new(
        cursor.get_f32_le(),
        cursor.get_f32_le(),
        cursor.get_f32_le(),
        cursor.get_f32_le(),
    )
}

/// Decode the wheel speed of the player's car
fn decode_wheel_speed(cursor: &mut Cursor<&mut BytesMut>) -> CornerProperty<f32> {
    CornerProperty::new(
        cursor.get_f32_le(),
        cursor.get_f32_le(),
        cursor.get_f32_le(),
        cursor.get_f32_le(),
    )
}

/// Decode the wheel slip of the player's car
fn decode_wheel_slip(cursor: &mut Cursor<&mut BytesMut>) -> CornerProperty<f32> {
    CornerProperty::new(
        cursor.get_f32_le(),
        cursor.get_f32_le(),
        cursor.get_f32_le(),
        cursor.get_f32_le(),
    )
}

/// Decode the local velocity of the player's car
fn decode_local_velocity(cursor: &mut Cursor<&mut BytesMut>) -> Property3D<f32> {
    Property3D::new(
        cursor.get_f32_le(),
        cursor.get_f32_le(),
        cursor.get_f32_le(),
    )
}

/// Decode the angular velocity of the player's car
fn decode_angular_velocity(cursor: &mut Cursor<&mut BytesMut>) -> Property3D<f32> {
    Property3D::new(
        cursor.get_f32_le(),
        cursor.get_f32_le(),
        cursor.get_f32_le(),
    )
}
/// Decode the angular acceleration of the player's car
fn decode_angular_acceleration(cursor: &mut Cursor<&mut BytesMut>) -> Property3D<f32> {
    Property3D::new(
        cursor.get_f32_le(),
        cursor.get_f32_le(),
        cursor.get_f32_le(),
    )
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use assert_approx_eq::assert_approx_eq;
    use bytes::{BufMut, BytesMut};

    use crate::nineteen::motion::{decode_motion, PACKET_SIZE};

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
    fn decode_motion_with_error() {
        let mut bytes = BytesMut::with_capacity(0);
        let mut cursor = Cursor::new(&mut bytes);

        let packet = decode_motion(&mut cursor);
        assert!(packet.is_err());
    }

    #[test]
    fn decode_motion_with_success() {
        let mut bytes = BytesMut::with_capacity(PACKET_SIZE);
        bytes = put_packet_header(bytes);

        bytes.put_f32_le(1.0);
        bytes.put_f32_le(2.0);
        bytes.put_f32_le(3.0);
        bytes.put_f32_le(4.0);
        bytes.put_f32_le(5.0);
        bytes.put_f32_le(6.0);
        bytes.put_i16_le(7);
        bytes.put_i16_le(8);
        bytes.put_i16_le(9);
        bytes.put_i16_le(10);
        bytes.put_i16_le(11);
        bytes.put_i16_le(12);
        bytes.put_f32_le(13.0);
        bytes.put_f32_le(14.0);
        bytes.put_f32_le(15.0);
        bytes.put_f32_le(16.0);
        bytes.put_f32_le(17.0);
        bytes.put_f32_le(18.0);

        let padding = vec![0u8; 1140];
        bytes.put(padding.as_slice());

        bytes.put_f32_le(19.0);
        bytes.put_f32_le(20.0);
        bytes.put_f32_le(21.0);
        bytes.put_f32_le(22.0);
        bytes.put_f32_le(23.0);
        bytes.put_f32_le(24.0);
        bytes.put_f32_le(25.0);
        bytes.put_f32_le(26.0);
        bytes.put_f32_le(27.0);
        bytes.put_f32_le(28.0);
        bytes.put_f32_le(29.0);
        bytes.put_f32_le(30.0);
        bytes.put_f32_le(31.0);
        bytes.put_f32_le(32.0);
        bytes.put_f32_le(33.0);
        bytes.put_f32_le(34.0);
        bytes.put_f32_le(35.0);
        bytes.put_f32_le(36.0);
        bytes.put_f32_le(37.0);
        bytes.put_f32_le(38.0);
        bytes.put_f32_le(39.0);
        bytes.put_f32_le(40.0);
        bytes.put_f32_le(41.0);
        bytes.put_f32_le(42.0);
        bytes.put_f32_le(43.0);
        bytes.put_f32_le(44.0);
        bytes.put_f32_le(45.0);
        bytes.put_f32_le(46.0);
        bytes.put_f32_le(47.0);
        bytes.put_f32_le(48.0);

        let mut cursor = Cursor::new(&mut bytes);
        let packet = decode_motion(&mut cursor).unwrap();

        let motion = packet.cars()[0];
        assert_approx_eq!(1.0, motion.position().x());
        assert_approx_eq!(4.0, motion.velocity().x());
        assert_eq!(7, motion.forward_direction().x());
        assert_eq!(10, motion.right_direction().x());
        assert_approx_eq!(13.0, motion.g_force().x());
        assert_approx_eq!(16.0, motion.yaw());
        assert_approx_eq!(17.0, motion.pitch());
        assert_approx_eq!(18.0, motion.roll());
        assert_approx_eq!(19.0, packet.suspension_position().front_left());
        assert_approx_eq!(23.0, packet.suspension_velocity().front_left());
        assert_approx_eq!(27.0, packet.suspension_acceleration().front_left());
        assert_approx_eq!(31.0, packet.wheel_speed().front_left());
        assert_approx_eq!(35.0, packet.wheel_slip().front_left());
        assert_approx_eq!(39.0, packet.local_velocity().x());
        assert_approx_eq!(42.0, packet.angular_velocity().x());
        assert_approx_eq!(45.0, packet.angular_acceleration().x());
        assert_approx_eq!(48.0, packet.front_wheels_angle());
    }
}
