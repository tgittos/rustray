use crate::core::vec;

pub trait Texturable {
    /// Returns the texture color value at the given coordinates and point.
    fn value(&self, u: f32, v: f32, p: &vec::Point3) -> vec::Vec3;
}
