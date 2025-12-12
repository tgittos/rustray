use std::any::Any;

use crate::math::vec;
use crate::traits::hittable;

pub trait Texturable: Any {
    /// Returns the texture color value at the given coordinates and point.
    fn sample(&self, hit_record: &hittable::Hit) -> vec::Vec3;

    fn as_any(&self) -> &dyn Any;
}
