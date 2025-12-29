use std::any::Any;
use crate::math::vec;

pub trait Sampleable {
    fn sample(&self) -> vec::Vec3;
    fn as_any(&self) -> &dyn Any;
}