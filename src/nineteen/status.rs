//! Decoder for car status packets sent by F1 2019
//!
//! F1 2019 is the first game to differentiate between a physical tyre compound (e.g. C1) and a
//! visual tyre compound (e.g. hard). This makes it packet format and decoder incompatible with
//! earlier F1 games.

use std::io::{Cursor, Error, ErrorKind};

use bytes::{Buf, BytesMut};

use crate::nineteen::flag::decode_flag;
use crate::nineteen::header::decode_header;
use crate::packet::ensure_packet_size;
use crate::packet::status::{
    CarStatus, CarStatusPacket, DrsSetting, ErsDeployMode, FuelMix, PhysicalTyreCompound,
    TractionControl, VisualTyreCompound,
};
use crate::types::CornerProperty;

/// Size of the car status packet in bytes
pub const PACKET_SIZE: usize = 1143;

/// Decode the car status packet sent by F1 2019
///
/// The car status packet by F1 2019 introduces the differentiation between a physical and a visual
/// tyre compound.
pub fn decode_statuses(cursor: &mut Cursor<&mut BytesMut>) -> Result<CarStatusPacket, Error> {
    ensure_packet_size(PACKET_SIZE, cursor)?;

    let header = decode_header(cursor)?;
    let mut car_status = Vec::with_capacity(20);

    for _ in 0..20 {
        car_status.push(CarStatus::new(
            decode_traction_control(cursor)?,
            cursor.get_u8() > 0,
            decode_fuel_mix(cursor)?,
            cursor.get_u8(),
            cursor.get_u8() > 0,
            cursor.get_f32_le(),
            cursor.get_f32_le(),
            cursor.get_f32_le(),
            cursor.get_u16_le(),
            cursor.get_u16_le(),
            cursor.get_u8(),
            decode_drs(cursor)?,
            decode_tyre_wear(cursor),
            decode_physical_tyre_compound(cursor)?,
            decode_visual_tyre_compound(cursor)?,
            decode_tyre_damage(cursor),
            cursor.get_u8(),
            cursor.get_u8(),
            cursor.get_u8(),
            cursor.get_u8(),
            cursor.get_u8(),
            decode_flag(cursor)?,
            cursor.get_f32_le(),
            decode_ers_deploy_mode(cursor)?,
            cursor.get_f32_le(),
            cursor.get_f32_le(),
            cursor.get_f32_le(),
        ));
    }

    Ok(CarStatusPacket::new(header, car_status))
}

fn decode_traction_control(cursor: &mut Cursor<&mut BytesMut>) -> Result<TractionControl, Error> {
    let value = cursor.get_u8();

    match value {
        0 => Ok(TractionControl::Off),
        1 => Ok(TractionControl::Low),
        2 => Ok(TractionControl::High),
        _ => Err(Error::new(
            ErrorKind::InvalidData,
            "Failed to decode transaction control.",
        )),
    }
}

fn decode_fuel_mix(cursor: &mut Cursor<&mut BytesMut>) -> Result<FuelMix, Error> {
    let value = cursor.get_u8();

    match value {
        0 => Ok(FuelMix::Lean),
        1 => Ok(FuelMix::Standard),
        2 => Ok(FuelMix::Rich),
        3 => Ok(FuelMix::Max),
        _ => Err(Error::new(
            ErrorKind::InvalidData,
            "Failed to decode fuel mix.",
        )),
    }
}

fn decode_drs(cursor: &mut Cursor<&mut BytesMut>) -> Result<DrsSetting, Error> {
    let value = cursor.get_i8();

    match value {
        -1 => Ok(DrsSetting::Unknown),
        0 => Ok(DrsSetting::NotAllowed),
        1 => Ok(DrsSetting::Allowed),
        _ => Err(Error::new(
            ErrorKind::InvalidData,
            "Failed to decode DRS status.",
        )),
    }
}

fn decode_tyre_wear(cursor: &mut Cursor<&mut BytesMut>) -> CornerProperty<u8> {
    CornerProperty::new(
        cursor.get_u8(),
        cursor.get_u8(),
        cursor.get_u8(),
        cursor.get_u8(),
    )
}

fn decode_physical_tyre_compound(
    cursor: &mut Cursor<&mut BytesMut>,
) -> Result<PhysicalTyreCompound, Error> {
    let value = cursor.get_u8();

    match value {
        7 => Ok(PhysicalTyreCompound::F1Intermediate),
        8 => Ok(PhysicalTyreCompound::F1Wet),
        9 => Ok(PhysicalTyreCompound::ClassicDry),
        10 => Ok(PhysicalTyreCompound::ClassicWet),
        11 => Ok(PhysicalTyreCompound::F2SuperSoft),
        12 => Ok(PhysicalTyreCompound::F2Soft),
        13 => Ok(PhysicalTyreCompound::F2Medium),
        14 => Ok(PhysicalTyreCompound::F2Hard),
        15 => Ok(PhysicalTyreCompound::F2Wet),
        16 => Ok(PhysicalTyreCompound::F1C5),
        17 => Ok(PhysicalTyreCompound::F1C4),
        18 => Ok(PhysicalTyreCompound::F1C3),
        19 => Ok(PhysicalTyreCompound::F1C2),
        20 => Ok(PhysicalTyreCompound::F1C1),
        _ => Err(Error::new(
            ErrorKind::InvalidData,
            "Failed to decode physical tyre compound.",
        )),
    }
}

fn decode_visual_tyre_compound(
    cursor: &mut Cursor<&mut BytesMut>,
) -> Result<VisualTyreCompound, Error> {
    let value = cursor.get_u8();

    match value {
        7 => Ok(VisualTyreCompound::F1Intermediate),
        8 => Ok(VisualTyreCompound::F1Wet),
        9 => Ok(VisualTyreCompound::ClassicDry),
        10 => Ok(VisualTyreCompound::ClassicWet),
        11 => Ok(VisualTyreCompound::F2SuperSoft),
        12 => Ok(VisualTyreCompound::F2Soft),
        13 => Ok(VisualTyreCompound::F2Medium),
        14 => Ok(VisualTyreCompound::F2Hard),
        15 => Ok(VisualTyreCompound::F2Wet),
        16 => Ok(VisualTyreCompound::F1Soft),
        17 => Ok(VisualTyreCompound::F1Medium),
        18 => Ok(VisualTyreCompound::F1Hard),
        _ => Err(Error::new(
            ErrorKind::InvalidData,
            "Failed to decode visual tyre compound.",
        )),
    }
}

fn decode_tyre_damage(cursor: &mut Cursor<&mut BytesMut>) -> CornerProperty<u8> {
    CornerProperty::new(
        cursor.get_u8(),
        cursor.get_u8(),
        cursor.get_u8(),
        cursor.get_u8(),
    )
}

fn decode_ers_deploy_mode(cursor: &mut Cursor<&mut BytesMut>) -> Result<ErsDeployMode, Error> {
    let value = cursor.get_u8();

    match value {
        0 => Ok(ErsDeployMode::None),
        1 => Ok(ErsDeployMode::Low),
        2 => Ok(ErsDeployMode::Medium),
        3 => Ok(ErsDeployMode::High),
        4 => Ok(ErsDeployMode::Overtake),
        5 => Ok(ErsDeployMode::Hotlap),
        _ => Err(Error::new(
            ErrorKind::InvalidData,
            "Failed to decode ERS deployment mode.",
        )),
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use assert_approx_eq::assert_approx_eq;
    use bytes::{BufMut, BytesMut};

    use crate::nineteen::status::{decode_statuses, PACKET_SIZE};
    use crate::packet::status::{
        DrsSetting, ErsDeployMode, FuelMix, PhysicalTyreCompound, TractionControl,
        VisualTyreCompound,
    };
    use crate::types::Flag;

    fn put_packet_header(mut bytes: BytesMut) -> BytesMut {
        bytes.put_u16_le(2019);
        bytes.put_u8(1);
        bytes.put_u8(2);
        bytes.put_u8(3);
        bytes.put_u8(7);
        bytes.put_u64_le(u64::max_value());
        bytes.put_f32_le(1.0);
        bytes.put_u32_le(u32::max_value());
        bytes.put_u8(0);

        bytes
    }

    #[test]
    fn decode_statuses_with_error() {
        let mut bytes = BytesMut::with_capacity(0);
        let mut cursor = Cursor::new(&mut bytes);

        let packet = decode_statuses(&mut cursor);
        assert!(packet.is_err());
    }

    #[test]
    #[allow(clippy::cognitive_complexity)]
    fn decode_statuses_with_success() {
        let mut bytes = BytesMut::with_capacity(PACKET_SIZE);
        bytes = put_packet_header(bytes);

        for _ in 0..20 {
            bytes.put_u8(1);
            bytes.put_u8(1);
            bytes.put_u8(3);
            bytes.put_u8(4);
            bytes.put_u8(1);
            bytes.put_f32_le(6.0);
            bytes.put_f32_le(7.0);
            bytes.put_f32_le(8.0);
            bytes.put_u16_le(9);
            bytes.put_u16_le(10);
            bytes.put_u8(11);
            bytes.put_i8(-1);
            bytes.put_u8(13);
            bytes.put_u8(14);
            bytes.put_u8(15);
            bytes.put_u8(16);
            bytes.put_u8(17);
            bytes.put_u8(18);
            bytes.put_u8(19);
            bytes.put_u8(20);
            bytes.put_u8(21);
            bytes.put_u8(22);
            bytes.put_u8(23);
            bytes.put_u8(24);
            bytes.put_u8(25);
            bytes.put_u8(26);
            bytes.put_u8(27);
            bytes.put_i8(-1);
            bytes.put_f32_le(29.0);
            bytes.put_u8(5);
            bytes.put_f32_le(31.0);
            bytes.put_f32_le(32.0);
            bytes.put_f32_le(33.0);
        }

        let mut cursor = Cursor::new(&mut bytes);

        let packet = decode_statuses(&mut cursor).unwrap();
        let status = packet.statuses()[0];

        assert_eq!(TractionControl::Low, status.traction_control());
        assert!(status.abs());
        assert_eq!(FuelMix::Max, status.fuel_mix());
        assert_eq!(4, status.brake_bias());
        assert!(status.pit_limiter());
        assert_approx_eq!(6.0, status.fuel_remaining());
        assert_approx_eq!(7.0, status.fuel_capacity());
        assert_approx_eq!(8.0, status.fuel_remaining_laps());
        assert_eq!(9, status.max_rpm());
        assert_eq!(10, status.idle_rpm());
        assert_eq!(11, status.gear_count());
        assert_eq!(DrsSetting::Unknown, status.drs());
        assert_eq!(13, status.tyre_wear().front_left());
        assert_eq!(PhysicalTyreCompound::F1C4, status.physical_tyre_compound());
        assert_eq!(VisualTyreCompound::F1Hard, status.visual_tyre_compound());
        assert_eq!(19, status.tyre_damage().front_left());
        assert_eq!(23, status.front_left_wing_damage());
        assert_eq!(24, status.front_right_wing_damage());
        assert_eq!(25, status.rear_wing_damage());
        assert_eq!(26, status.engine_damage());
        assert_eq!(27, status.gear_box_damage());
        assert_eq!(Flag::Invalid, status.vehicle_flags());
        assert_approx_eq!(29.0, status.ers_energy());
        assert_eq!(ErsDeployMode::Hotlap, status.ers_deploy_mode());
        assert_approx_eq!(31.0, status.ers_harvest_mgu_k());
        assert_approx_eq!(32.0, status.ers_harvest_mgu_h());
        assert_approx_eq!(33.0, status.ers_deployed());
    }
}
