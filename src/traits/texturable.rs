use crate::core::vec;
use crate::traits::hittable;

#[typetag::serde(tag = "texturable")]
pub trait Texturable {
    /// Returns the texture color value at the given coordinates and point.
    fn sample(&self, hit_record: &hittable::Hit) -> vec::Vec3;
}
