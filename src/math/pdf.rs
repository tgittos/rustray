use crate::math::vec;

/// Probability Density Function trait
pub trait PDF {
    fn value(&self, direction: vec::Vec3) -> f32;
    fn generate(&self, rng: &mut rand::rngs::ThreadRng) -> vec::Vec3;
}
