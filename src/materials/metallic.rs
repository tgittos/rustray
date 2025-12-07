//! Reflective metallic material with optional roughness for blurred reflections.
use std::time;

use crate::core::{ray, scene, vec};
use crate::traits::hittable;
use crate::traits::renderable::Renderable;
use crate::traits::sampleable;
use crate::utils::stats;

/// Mirror-like surface with an albedo tint and surface roughness.
pub struct Metallic {
    pub albedo: vec::Vec3,
    pub roughness: f32,
}

impl Metallic {
    /// Creates a metallic material; roughness is clamped to `[0, 1]`.
    pub fn new(albedo: &vec::Vec3, roughness: f32) -> Self {
        Metallic {
            albedo: *albedo,
            roughness: if roughness < 1.0 { roughness } else { 1.0 },
        }
    }
}

/// Samples a specular reflection with optional fuzziness.
fn metallic_sample(
    metallic: &Metallic,
    rng: &mut rand::rngs::ThreadRng,
    hit_record: &hittable::HitRecord,
    scene: &scene::Scene,
    depth: u32,
) -> vec::Vec3 {
    if depth == 0 {
        return vec::Vec3::new(0.0, 0.0, 0.0);
    }

    let hit = hit_record.hit;
    let reflected = vec::reflect(&vec::unit_vector(&hit.ray.direction), &hit.normal);
    let scattered = ray::Ray::new(
        &hit.point,
        &(reflected + vec::random_in_unit_sphere(rng) * metallic.roughness),
        Some(hit.ray.time),
    );

    let mut new_hit_record: Option<hittable::HitRecord> = None;

    let hit_start = time::Instant::now();
    if let Some(record) = scene.hit(&scattered, 0.001, f32::MAX) {
        new_hit_record = Some(record);
    }
    stats::add_hit_stat(stats::Stat::new(stats::METALLIC_HIT, hit_start.elapsed()));

    if new_hit_record.is_none() {
        return vec::Vec3::new(0.0, 0.0, 0.0);
    }

    let new_hit_record = new_hit_record.unwrap();
    let sample_start = time::Instant::now();
    let bounce = new_hit_record
        .renderable
        .sample(rng, &new_hit_record, scene, depth - 1);

    stats::add_sample_stat(stats::Stat::new(
        stats::METALLIC_SAMPLE,
        sample_start.elapsed(),
    ));

    return metallic.albedo * bounce;
}

impl sampleable::Sampleable for Metallic {
    fn sample(
        &self,
        rng: &mut rand::rngs::ThreadRng,
        hit_record: &hittable::HitRecord,
        scene: &scene::Scene,
        depth: u32,
    ) -> vec::Vec3 {
        metallic_sample(self, rng, hit_record, scene, depth)
    }
}

impl sampleable::Sampleable for &Metallic {
    fn sample(
        &self,
        rng: &mut rand::rngs::ThreadRng,
        hit_record: &hittable::HitRecord,
        scene: &scene::Scene,
        depth: u32,
    ) -> vec::Vec3 {
        metallic_sample(self, rng, hit_record, scene, depth)
    }
}
