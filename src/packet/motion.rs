//! Motion data on all cars in the session
//!
//! The F1 games provide data about the position and movement of each car in the session in the
//! motion packet. The rate with which these packets are sent can be configured in the game. F1 2018
//! and F1 2019 publish the same motion data.

use crate::packet::header::Header;
use crate::types::{CornerProperty, Property3D};
use derive_new::new;
use getset::{CopyGetters, Getters};

/// Data about a car and its position and movement in space
///
/// The position and movement of each car in a session is described in the motion packet.
#[derive(new, Debug, CopyGetters, Getters, PartialEq, Copy, Clone, PartialOrd, Default)]
#[allow(clippy::too_many_arguments)]
pub struct Motion {
    /// Returns the position of the car in 3D space.
    #[getset(get = "pub")]
    position: Property3D<f32>,

    /// Returns the velocity of the car on three axis.
    #[getset(get = "pub")]
    velocity: Property3D<f32>,

    /// Returns the normalized forward motion of the car on three axis.
    ///
    /// Normalized values can be converted to float through division by 32767.0f.
    #[getset(get = "pub")]
    forward_direction: Property3D<i16>,

    /// Returns the normalized lateral motion of the car on three axis.
    ///
    /// Normalized values can be converted to float through division by 32767.0f.
    #[getset(get = "pub")]
    right_direction: Property3D<i16>,

    /// Returns the G force on the car on each of the three axis.
    #[getset(get = "pub")]
    g_force: Property3D<f32>,

    /// Returns the yaw angle of the car in radians.
    #[getset(get_copy = "pub")]
    yaw: f32,

    /// Returns the pitch angle of the car in radians.
    #[getset(get_copy = "pub")]
    pitch: f32,

    /// Returns the roll angle of the car in radians.
    #[getset(get_copy = "pub")]
    roll: f32,
}

/// Packet containing data about the movement and position of all cars in the session
///
/// The F1 games publish motion data for all cars in the session. This data is restricted to
/// publicly observable properties for most cars, e.g. the position and movement of a car. For the
/// player's car, additional motion data is published, e.g. various physical forces on the car and
/// its suspension.
#[derive(new, Debug, CopyGetters, Getters, PartialEq, Clone, PartialOrd, Default)]
#[allow(clippy::too_many_arguments)]
pub struct MotionPacket {
    /// Returns the packet header prefixing the motion packet.
    #[getset(get = "pub")]
    header: Header,

    /// Returns the publicly observable motion data for all 20 cars in the session.
    #[getset(get = "pub")]
    cars: Vec<Motion>,

    /// Returns the position of the suspension at each corner of the car.
    #[getset(get = "pub")]
    suspension_positions: CornerProperty<f32>,

    /// Returns the velocity of the suspension at each corner of the car.
    #[getset(get = "pub")]
    suspension_velocity: CornerProperty<f32>,

    /// Returns the acceleration of the suspension at each corner of the car.
    #[getset(get = "pub")]
    suspension_acceleration: CornerProperty<f32>,

    /// Returns the wheel speed at each corner of the car.
    #[getset(get = "pub")]
    wheel_speed: CornerProperty<f32>,

    /// Returns the wheel slip at each corner of the car.
    #[getset(get = "pub")]
    wheel_slip: CornerProperty<f32>,

    /// Returns the velocity in local space on each axis.
    #[getset(get = "pub")]
    local_velocity: Property3D<f32>,

    /// Returns the angular velocity on each axis.
    #[getset(get = "pub")]
    angular_velocity: Property3D<f32>,

    /// Returns the angular acceleration on each axis.
    #[getset(get = "pub")]
    angular_acceleration: Property3D<f32>,

    /// Returns the current angle of the front wheels in radians.
    #[getset(get_copy = "pub")]
    front_wheels_angle: f32,
}
