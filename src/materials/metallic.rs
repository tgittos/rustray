//! Reflective metallic material with optional roughness for blurred reflections.
use serde::{Deserialize, Serialize};

use crate::core::ray;
use crate::math::vec;
use crate::traits::hittable;
use crate::traits::scatterable::{ScatterRecord, Scatterable};

/// Mirror-like surface with an albedo tint and surface roughness.
#[derive(Clone, Serialize, Deserialize)]
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

impl Scatterable for Metallic {
    /// Samples a specular reflection with optional fuzziness.
    fn scatter(
        &self,
        rng: &mut rand::rngs::ThreadRng,
        hit_record: &hittable::HitRecord,
        depth: u32,
    ) -> Option<ScatterRecord> {
        if depth == 0 {
            return None;
        }

        let hit = hit_record.hit;
        let reflected = vec::reflect(&vec::unit_vector(&hit.ray.direction), &hit.normal);
        let scattered_ray = ray::Ray::new(
            &hit.point,
            &(reflected + vec::random_in_unit_sphere(rng) * self.roughness),
            Some(hit.ray.time),
        );

        Some(ScatterRecord {
            attenuation: self.albedo,
            scatter_pdf: None,
            scattered_ray: Some(scattered_ray),
            use_light_pdf: false,
        })
    }

    fn emit(&self, _hit_record: &hittable::HitRecord) -> vec::Vec3 {
        vec::Vec3::new(0.0, 0.0, 0.0)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
