//! Transparent material that refracts and reflects based on a refractive index.
use rand::Rng;

use crate::core::{ray, scene, vec};
use crate::traits::hittable;
use crate::traits::renderable::Renderable;
use crate::traits::sampleable::Sampleable;

/// Glass-like dielectric material with a configurable refractive index.
pub struct Dielectric {
    pub refractive_index: f32,
}

impl Dielectric {
    /// Builds a new dielectric material (e.g., 1.5 for glass).
    pub fn new(refractive_index: f32) -> Self {
        Dielectric { refractive_index }
    }
}

/// Samples a ray through reflection or refraction using Schlick's approximation.
fn dielectric_sample(
    dielectric: &Dielectric,
    rng: &mut rand::rngs::ThreadRng,
    hit_record: &hittable::HitRecord,
    scene: &scene::Scene,
    depth: u32,
) -> vec::Vec3 {
    let hit = hit_record.hit;
    let unit_direction = vec::unit_vector(&hit.ray.direction);

    // Orient the normal against the incoming ray so refraction math is stable.
    let front_face = unit_direction.dot(&hit.normal) < 0.0;
    let normal = if front_face { hit.normal } else { -hit.normal };
    let refraction_ratio = if front_face {
        1.0 / dielectric.refractive_index
    } else {
        dielectric.refractive_index
    };

    let cos_theta = (-unit_direction.dot(&normal)).min(1.0);
    let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

    let cannot_refract = refraction_ratio * sin_theta > 1.0;
    let reflectance = {
        let r0 =
            ((1.0 - dielectric.refractive_index) / (1.0 + dielectric.refractive_index)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cos_theta).powi(5)
    };

    let scatter_direction = if cannot_refract || rng.random::<f32>() < reflectance {
        vec::reflect(&unit_direction, &normal)
    } else {
        vec::refract(&unit_direction, &normal, refraction_ratio).unwrap()
    };

    let attenuation = vec::Vec3::new(1.0, 1.0, 1.0);

    if depth == 0 {
        return vec::Vec3::new(0.0, 0.0, 0.0);
    }

    if let Some(new_hit_record) = scene.hit(
        &ray::Ray::new(&hit.point, &scatter_direction, Some(hit.ray.time)),
        0.001,
        f32::MAX,
    ) {
        let bounce = new_hit_record
            .renderable
            .sample(rng, &new_hit_record, scene, depth - 1);
        attenuation * bounce
    } else {
        vec::Vec3::new(0.0, 0.0, 0.0)
    }
}

impl Sampleable for Dielectric {
    fn sample(
        &self,
        rng: &mut rand::rngs::ThreadRng,
        hit_record: &hittable::HitRecord,
        scene: &scene::Scene,
        depth: u32,
    ) -> vec::Vec3 {
        dielectric_sample(self, rng, hit_record, scene, depth)
    }
}

impl Sampleable for &Dielectric {
    fn sample(
        &self,
        rng: &mut rand::rngs::ThreadRng,
        hit_record: &hittable::HitRecord,
        scene: &scene::Scene,
        depth: u32,
    ) -> vec::Vec3 {
        dielectric_sample(self, rng, hit_record, scene, depth)
    }
}
