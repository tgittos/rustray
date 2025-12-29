use rand::rngs;

use crate::math::{pdf, vec};

pub struct ConstantPhaseFunction {}

impl pdf::PDF for ConstantPhaseFunction {
    fn value(&self, _direction: vec::Vec3) -> f32 {
        1.0 / (4.0 * std::f32::consts::PI)
    }

    fn generate(&self, rng: &mut rngs::ThreadRng) -> vec::Vec3 {
        vec::random_in_unit_sphere(rng)
    }
}
