//! Lambertian diffuse material that scatters light uniformly.
use serde::{Deserialize, Serialize};
use std::time;

use crate::core::{ray, scene, vec};
use crate::traits::renderable::Renderable;
use crate::traits::{hittable, sampleable, texturable};
use crate::utils::stats;

/// Diffuse surface with a constant albedo.
#[derive(Serialize, Deserialize)]
pub struct Lambertian {
    pub texture: Box<dyn texturable::Texturable>,
}

impl Lambertian {
    /// Creates a new diffuse material with the given albedo.
    pub fn new(texture: Box<dyn texturable::Texturable>) -> Self {
        Self { texture }
    }
}

#[typetag::serde]
impl sampleable::Sampleable for Lambertian {
    /// Samples a diffuse bounce using cosine-weighted hemisphere sampling.
    fn sample(
        &self,
        rng: &mut rand::rngs::ThreadRng,
        hit_record: &hittable::HitRecord,
        scene: &scene::Scene,
        depth: u32,
    ) -> vec::Vec3 {
        if depth == 0 {
            return vec::Vec3::new(0.0, 0.0, 0.0);
        }

        let hit = hit_record.hit;
        let target = hit.point + hit.normal + vec::random_in_unit_sphere(rng);

        // bounce ray and attenuate
        let new_ray = ray::Ray::new(&hit.point, &(target - hit.point), Some(hit.ray.time));
        let hit_start = time::Instant::now();
        if let Some(new_hit_record) = scene.hit(&new_ray, 0.001, f32::MAX) {
            stats::add_hit_stat(stats::Stat::new(stats::LAMBERTIAN_HIT, hit_start.elapsed()));
            let sample_start = time::Instant::now();
            let bounce = new_hit_record
                .renderable
                .sample(rng, &new_hit_record, scene, depth - 1);
            stats::add_sample_stat(stats::Stat::new(
                stats::LAMBERTIAN_SAMPLE,
                sample_start.elapsed(),
            ));
            return self.texture.sample(&hit_record.hit) * (0.5 * bounce);
        }

        // miss
        vec::Vec3::new(0.0, 0.0, 0.0)
    }
}
