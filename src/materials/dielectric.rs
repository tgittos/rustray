//! Transparent material that refracts and reflects based on a refractive index.
use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::core::ray;
use crate::math::vec;
use crate::traits::hittable;
use crate::traits::scatterable::{ScatterRecord, Scatterable};

/// Glass-like dielectric material with a configurable refractive index.
#[derive(Clone, Serialize, Deserialize)]
pub struct Dielectric {
    pub refractive_index: f32,
}

impl Dielectric {
    /// Builds a new dielectric material (e.g., 1.5 for glass).
    pub fn new(refractive_index: f32) -> Self {
        Dielectric { refractive_index }
    }
}

impl Scatterable for Dielectric {
    fn scatter(
        &self,
        rng: &mut rand::rngs::ThreadRng,
        hit_record: &hittable::HitRecord,
        depth: u32,
    ) -> Option<ScatterRecord> {
        let hit = hit_record.hit;
        let unit_direction = vec::unit_vector(&hit.ray.direction);

        // Orient the normal against the incoming ray so refraction math is stable.
        let front_face = unit_direction.dot(&hit.normal) < 0.0;
        let normal = if front_face { hit.normal } else { -hit.normal };
        let refraction_ratio = if front_face {
            1.0 / self.refractive_index
        } else {
            self.refractive_index
        };

        let cos_theta = (-unit_direction.dot(&normal)).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let reflectance = {
            let r0 = ((1.0 - self.refractive_index) / (1.0 + self.refractive_index)).powi(2);
            r0 + (1.0 - r0) * (1.0 - cos_theta).powi(5)
        };

        let scatter_direction = if cannot_refract || rng.random::<f32>() < reflectance {
            vec::reflect(&unit_direction, &normal)
        } else {
            let refracted = vec::refract(&unit_direction, &normal, refraction_ratio);
            match refracted {
                Some(r) => r,
                None => vec::reflect(&unit_direction, &normal),
            }
        };

        let attenuation = vec::Vec3::new(1.0, 1.0, 1.0);

        if depth == 0 {
            return None;
        }

        let scattered_ray = ray::Ray::new(&hit.point, &scatter_direction, Some(hit.ray.time));

        Some(ScatterRecord {
            attenuation,
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
