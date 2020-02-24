use crate::nineteen::PacketHeader;

pub struct CarMotion {
    /// The position in the world on the X, Y, and Z axis.
    world_position: (f32, f32, f32),

    /// The velocity in the world on the X, Y, and Z axis.
    world_velocity: (f32, f32, f32),

    /// The direction of the forward motion on the X, Y, and Z axis. This value
    /// is normalized. To convert to float, divide by 32767.0f.
    world_forward_direction: (i16, i16, i16),

    /// The direction of lateral motion on the X, Y, and Z axis. This value is
    /// normalized. To convert to float, divide by 32767.0f.
    world_right_direction: (i16, i16, i16),

    /// The G force, separated in its lateral, longitudinal, and vertical
    /// components.
    gforce: (f32, f32, f32),

    /// The yaw angle of the car in radians.
    yaw: f32,

    /// The pitch angle of the car in radians.
    pitch: f32,

    /// The roll angle of the car in radians.
    roll: f32,
}

pub struct MotionPacket {
    /// Each packet starts with a packet header.
    pub header: PacketHeader,

    /// Motion data for all cars on track.
    pub cars: Vec<CarMotion>,

    /// Position of the suspension at the RL, RR, FL, FR.
    suspension_positions: (f32, f32, f32, f32),

    /// Velocity of the suspension at the RL, RR, FL, FR.
    suspension_velocity: (f32, f32, f32, f32),

    /// Acceleration of the suspension at the RL, RR, FL, FR.
    suspension_acceleration: (f32, f32, f32, f32),

    /// Wheel sped at the RL, RR, FL, FR.
    wheel_speed: (f32, f32, f32, f32),

    /// Wheel slip at the RL, RR, FL, FR.
    wheel_slip: (f32, f32, f32, f32),

    /// Velocity in local space on the X, Y, and Z axis.
    local_velocity: (f32, f32, f32),

    /// Angular velocity on the X, Y, and Z axis.
    angular_velocity: (f32, f32, f32),

    /// Angular acceleration on the X, Y, and Z axis.
    angular_acceleration: (f32, f32, f32),

    /// Current angle of the front wheels in radians.
    front_wheels_angle: f32,
}
