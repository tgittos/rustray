//! Traits for materials that can produce color contributions from ray hits.
use std::any::Any;

use crate::core::scene;
use crate::math::vec;
use crate::traits::hittable;

/// Trait for objects that can be sampled for color contribution.
pub trait Sampleable: Any {
    /// Samples the color contribution at the hit point.
    fn sample(
        &self,
        rng: &mut rand::rngs::ThreadRng,
        hit_record: &hittable::HitRecord,
        scene: &scene::Scene,
        depth: u32,
    ) -> vec::Vec3;

    fn as_any(&self) -> &dyn Any;
}
