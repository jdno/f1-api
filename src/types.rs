use derive_new::new;
use getset::CopyGetters;

/// Reference to a vehicle in a packet
///
/// In Formula 1, a maximum of 20 cars can participate in any session. The modern F1 games use this
/// rule to use arrays with a static size of 20 whenever they publish data about all vehicles in a
/// session. Data in those arrays is referenced using an unsigned byte. By defining a type alias for
/// the indices, their usage can be checked by the Rust compiler.
pub type VehicleIndex = u8;

/// Property on each corner of a car
///
/// The F1 games publish telemetry data and setup parameters that describe each corner of a car. For
/// example, the suspension settings or tyre pressures are a set of four numbers, one for each
/// corner of the car. These properties can be expressed by the `Corner` type.
///
/// # Examples
///
/// ```
/// use f1_api::types::CornerProperty;
///
/// let suspension_position = CornerProperty::new(1.0, 0.9, 1.1, 1.0);
/// ```
#[derive(new, Debug, CopyGetters, PartialEq, Copy, Clone, Eq, Ord, PartialOrd, Hash, Default)]
pub struct CornerProperty<T>
where
    T: Copy,
{
    /// Returns the value of the property at the front left.
    #[getset(get_copy = "pub")]
    front_left: T,

    /// Returns the value of the property at the front right.
    #[getset(get_copy = "pub")]
    front_right: T,

    /// Returns the value of the property at the rear left.
    #[getset(get_copy = "pub")]
    rear_left: T,

    /// Returns the value of the property at the rear right.
    #[getset(get_copy = "pub")]
    rear_right: T,
}

/// Property in a three-dimensional world
///
/// The F1 games publish data that places objects in a three dimensional world. Examples include the
/// position of a car, its velocity, or the direction of its motion.
///
/// # Examples
///
/// ```
/// use f1_api::types::Property3D;
///
/// let g_forces = Property3D::new(1.0, 1.3, 0.9);
/// ```
#[derive(new, Debug, CopyGetters, PartialEq, Copy, Clone, Eq, Ord, PartialOrd, Hash, Default)]
pub struct Property3D<T>
where
    T: Copy,
{
    /// Returns the component on the X axis of the three-dimensional property.
    #[getset(get_copy = "pub")]
    x: T,

    /// Returns the component on the Y axis of the three-dimensional property.
    #[getset(get_copy = "pub")]
    y: T,

    /// Returns the component on the Z axis of the three-dimensional property.
    #[getset(get_copy = "pub")]
    z: T,
}
