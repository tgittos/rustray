//! Traits for materials that can produce color contributions from ray hits.
use crate::core::{scene, vec};
use crate::traits::hittable;

/// Trait for objects that can be sampled for color contribution.
pub trait Sampleable {
    /// Samples the color contribution at the hit point.
    ///
    /// # Arguments
    /// * `rng` - A mutable reference to a random number generator.
    /// * [`hittable::HitRecord`] `hit_record` - The hit record containing information about the intersection.
    /// * [`scene::Scene`] `scene` - The scene containing all objects.
    /// * `depth` - The current recursion depth.
    ///
    /// # Returns
    /// A [`vec::Vec3`] Vec3 representing the color contribution.
    fn sample(
        &self,
        rng: &mut rand::rngs::ThreadRng,
        hit_record: &hittable::HitRecord,
        scene: &scene::Scene,
        depth: u32,
    ) -> vec::Vec3;
}
