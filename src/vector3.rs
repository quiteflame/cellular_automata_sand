#[derive(Clone, PartialOrd, PartialEq, Debug, Copy, Default)]
pub struct Vector3<T> {
    /// X coordinate of the vector.
    pub x: T,
    /// Y coordinate of the vector.
    pub y: T,
    /// Z coordinate of the vector.
    pub z: T,
}

impl<T> Vector3<T> {
    /// Create a new `Vector3` with the given values.
    pub const fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}
