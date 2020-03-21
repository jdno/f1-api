//! Decoder for car setup packets sent by F1 2019
//!
//! The car setup packets by F1 2018 and F1 2019 differ only in their packet headers, the rest of
//! the packet format is identical.

use crate::nineteen::header::decode_header;
use crate::packet::ensure_packet_size;
use crate::packet::setup::{CarSetup, CarSetupPacket};
use bytes::{Buf, BytesMut};
use std::io::{Cursor, Error};

/// Size of the car setups packet in bytes
pub const PACKET_SIZE: usize = 843;

/// Decode a car setup packet sent by F1 2019
///
/// F1 2018 and F1 2019 publish the same data in their car setup packets, but with different packet
/// headers. In multiplayer sessions, the setups of other players are redacted and appear empty.
pub fn decode_setups(cursor: &mut Cursor<&mut BytesMut>) -> Result<CarSetupPacket, Error> {
    ensure_packet_size(PACKET_SIZE, cursor)?;

    let header = decode_header(cursor)?;
    let mut setups = Vec::with_capacity(20);

    for _ in 0..20 {
        setups.push(CarSetup::new(
            cursor.get_u8(),
            cursor.get_u8(),
            cursor.get_u8(),
            cursor.get_u8(),
            cursor.get_f32_le(),
            cursor.get_f32_le(),
            cursor.get_f32_le(),
            cursor.get_f32_le(),
            cursor.get_u8(),
            cursor.get_u8(),
            cursor.get_u8(),
            cursor.get_u8(),
            cursor.get_u8(),
            cursor.get_u8(),
            cursor.get_u8(),
            cursor.get_u8(),
            cursor.get_f32_le(),
            cursor.get_f32_le(),
            cursor.get_u8(),
            cursor.get_f32_le(),
        ))
    }

    Ok(CarSetupPacket::new(header, setups))
}

#[cfg(test)]
mod tests {
    use crate::nineteen::setup::{decode_setups, PACKET_SIZE};
    use assert_approx_eq::assert_approx_eq;
    use bytes::{BufMut, BytesMut};
    use std::io::Cursor;

    fn put_packet_header(mut bytes: BytesMut) -> BytesMut {
        bytes.put_u16_le(2019);
        bytes.put_u8(1);
        bytes.put_u8(2);
        bytes.put_u8(3);
        bytes.put_u8(5);
        bytes.put_u64_le(u64::max_value());
        bytes.put_f32_le(1.0);
        bytes.put_u32_le(u32::max_value());
        bytes.put_u8(0);

        bytes
    }

    #[test]
    fn decode_setups_with_error() {
        let mut bytes = BytesMut::with_capacity(0);
        let mut cursor = Cursor::new(&mut bytes);

        let packet = decode_setups(&mut cursor);
        assert!(packet.is_err());
    }

    #[test]
    fn decode_setups_with_success() {
        let mut bytes = BytesMut::with_capacity(PACKET_SIZE);
        bytes = put_packet_header(bytes);

        bytes.put_u8(1);
        bytes.put_u8(2);
        bytes.put_u8(3);
        bytes.put_u8(4);
        bytes.put_f32_le(5.0);
        bytes.put_f32_le(6.0);
        bytes.put_f32_le(7.0);
        bytes.put_f32_le(8.0);
        bytes.put_u8(9);
        bytes.put_u8(10);
        bytes.put_u8(11);
        bytes.put_u8(12);
        bytes.put_u8(13);
        bytes.put_u8(14);
        bytes.put_u8(15);
        bytes.put_u8(16);
        bytes.put_f32_le(17.0);
        bytes.put_f32_le(18.0);
        bytes.put_u8(19);
        bytes.put_f32_le(20.0);

        let padding = vec![0u8; 779];
        bytes.put(padding.as_slice());

        let mut cursor = Cursor::new(&mut bytes);

        let packet = decode_setups(&mut cursor).unwrap();
        let setup = packet.setups()[0];

        assert_eq!(1, setup.front_wing());
        assert_eq!(2, setup.rear_wing());
        assert_eq!(3, setup.on_throttle());
        assert_eq!(4, setup.off_throttle());
        assert_approx_eq!(5.0, setup.front_camber());
        assert_approx_eq!(6.0, setup.rear_camber());
        assert_approx_eq!(7.0, setup.front_toe());
        assert_approx_eq!(8.0, setup.rear_toe());
        assert_eq!(9, setup.front_suspension());
        assert_eq!(10, setup.rear_suspension());
        assert_eq!(11, setup.front_anti_roll_bar());
        assert_eq!(12, setup.rear_anti_roll_bar());
        assert_eq!(13, setup.front_suspension_height());
        assert_eq!(14, setup.rear_suspension_height());
        assert_eq!(15, setup.brake_pressure());
        assert_eq!(16, setup.brake_bias());
        assert_approx_eq!(17.0, setup.front_tyre_pressure());
        assert_approx_eq!(18.0, setup.rear_tyre_pressure());
        assert_eq!(19, setup.ballast());
        assert_approx_eq!(20.0, setup.fuel_load());
    }
}
