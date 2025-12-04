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
    let reflected = vec::reflect(&vec::unit_vector(&hit.ray.direction), &hit.normal);
    let outward_normal;
    let ni_over_nt;
    let attenuation = vec::Vec3::new(1.0, 1.0, 1.0);
    let cosine;

    if hit.ray.direction.dot(&hit.normal) > 0.0 {
        // Ray is inside the material
        outward_normal = -hit.normal;
        ni_over_nt = dielectric.refractive_index;
        cosine = dielectric.refractive_index * hit.ray.direction.dot(&hit.normal)
            / hit.ray.direction.length();
    } else {
        // Ray is outside the material
        outward_normal = hit.normal;
        ni_over_nt = 1.0 / dielectric.refractive_index;
        cosine = -hit.ray.direction.dot(&hit.normal) / hit.ray.direction.length();
    }

    let refracted = vec::refract(
        &vec::unit_vector(&hit.ray.direction),
        &outward_normal,
        ni_over_nt,
    );
    let scatter_direction = match refracted {
        Some(refracted) => {
            let r0 =
                ((1.0 - dielectric.refractive_index) / (1.0 + dielectric.refractive_index)).powi(2);
            let reflect_prob = r0 + (1.0 - r0) * (1.0 - cosine).powi(5);
            if rng.random::<f32>() < reflect_prob {
                reflected
            } else {
                refracted
            }
        }
        None => {
            // Total internal reflection
            reflected
        }
    };

    if depth == 0 {
        return vec::Vec3::new(0.0, 0.0, 0.0);
    }

    if let Some(new_hit_record) = scene.hit(
        &ray::Ray::new(&hit.point, &scatter_direction),
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
