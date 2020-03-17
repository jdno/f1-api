//! Packet with motion data for all cars in the session

use crate::from_bytes::FromBytes;
use crate::nineteen::PacketHeader;
use bytes::{Buf, BytesMut};
use std::io::{Cursor, Error};

/// Public data describing the motion of each car in the session.
///
/// F1 2019 publishes a limited set of motion data for each car in a session. This data contains the
/// position of the car in the world, as all we its movement.
pub struct CarMotion {
    /// The position in the world on the X, Y, and Z axis.
    pub world_position: (f32, f32, f32),

    /// The velocity in the world on the X, Y, and Z axis.
    pub world_velocity: (f32, f32, f32),

    /// The direction of the forward motion on the X, Y, and Z axis. This value
    /// is normalized. To convert to float, divide by 32767.0f.
    pub world_forward_direction: (i16, i16, i16),

    /// The direction of lateral motion on the X, Y, and Z axis. This value is
    /// normalized. To convert to float, divide by 32767.0f.
    pub world_right_direction: (i16, i16, i16),

    /// The G force, separated in its lateral, longitudinal, and vertical
    /// components.
    pub gforce: (f32, f32, f32),

    /// The yaw angle of the car in radians.
    pub yaw: f32,

    /// The pitch angle of the car in radians.
    pub pitch: f32,

    /// The roll angle of the car in radians.
    pub roll: f32,
}

/// A packet with motion data about each car in the session.
///
/// F1 2019 publishes motion data for all cars in the session. For most cars, this is restricted to
/// publicly observable data, e.g. the position and movement of a car in the world. For the player's
/// car, more motion data is published, e.g. various physical forces on the car and its suspension.
pub struct MotionPacket {
    /// Each packet starts with a packet header.
    pub header: PacketHeader,

    /// Motion data for all cars on track.
    pub cars: Vec<CarMotion>,

    /// Position of the suspension at the RL, RR, FL, FR.
    pub suspension_positions: (f32, f32, f32, f32),

    /// Velocity of the suspension at the RL, RR, FL, FR.
    pub suspension_velocity: (f32, f32, f32, f32),

    /// Acceleration of the suspension at the RL, RR, FL, FR.
    pub suspension_acceleration: (f32, f32, f32, f32),

    /// Wheel sped at the RL, RR, FL, FR.
    pub wheel_speed: (f32, f32, f32, f32),

    /// Wheel slip at the RL, RR, FL, FR.
    pub wheel_slip: (f32, f32, f32, f32),

    /// Velocity in local space on the X, Y, and Z axis.
    pub local_velocity: (f32, f32, f32),

    /// Angular velocity on the X, Y, and Z axis.
    pub angular_velocity: (f32, f32, f32),

    /// Angular acceleration on the X, Y, and Z axis.
    pub angular_acceleration: (f32, f32, f32),

    /// Current angle of the front wheels in radians.
    pub front_wheels_angle: f32,
}

impl FromBytes for MotionPacket {
    fn buffer_size() -> usize {
        1343
    }

    fn decode(cursor: &mut Cursor<&mut BytesMut>) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let header = PacketHeader::decode(cursor)?;
        let mut cars = Vec::with_capacity(20);

        for _ in 0..20 {
            cars.push(CarMotion {
                world_position: (
                    cursor.get_f32_le(),
                    cursor.get_f32_le(),
                    cursor.get_f32_le(),
                ),
                world_velocity: (
                    cursor.get_f32_le(),
                    cursor.get_f32_le(),
                    cursor.get_f32_le(),
                ),
                world_forward_direction: (
                    cursor.get_i16_le(),
                    cursor.get_i16_le(),
                    cursor.get_i16_le(),
                ),
                world_right_direction: (
                    cursor.get_i16_le(),
                    cursor.get_i16_le(),
                    cursor.get_i16_le(),
                ),
                gforce: (
                    cursor.get_f32_le(),
                    cursor.get_f32_le(),
                    cursor.get_f32_le(),
                ),
                yaw: cursor.get_f32_le(),
                pitch: cursor.get_f32_le(),
                roll: cursor.get_f32_le(),
            })
        }

        Ok(MotionPacket {
            header,
            cars,
            suspension_positions: (
                cursor.get_f32_le(),
                cursor.get_f32_le(),
                cursor.get_f32_le(),
                cursor.get_f32_le(),
            ),
            suspension_velocity: (
                cursor.get_f32_le(),
                cursor.get_f32_le(),
                cursor.get_f32_le(),
                cursor.get_f32_le(),
            ),
            suspension_acceleration: (
                cursor.get_f32_le(),
                cursor.get_f32_le(),
                cursor.get_f32_le(),
                cursor.get_f32_le(),
            ),
            wheel_speed: (
                cursor.get_f32_le(),
                cursor.get_f32_le(),
                cursor.get_f32_le(),
                cursor.get_f32_le(),
            ),
            wheel_slip: (
                cursor.get_f32_le(),
                cursor.get_f32_le(),
                cursor.get_f32_le(),
                cursor.get_f32_le(),
            ),
            local_velocity: (
                cursor.get_f32_le(),
                cursor.get_f32_le(),
                cursor.get_f32_le(),
            ),
            angular_velocity: (
                cursor.get_f32_le(),
                cursor.get_f32_le(),
                cursor.get_f32_le(),
            ),
            angular_acceleration: (
                cursor.get_f32_le(),
                cursor.get_f32_le(),
                cursor.get_f32_le(),
            ),
            front_wheels_angle: cursor.get_f32_le(),
        })
    }
}
