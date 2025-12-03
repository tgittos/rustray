use rand::Rng;

use crate::core::vec;
use crate::core::ray;
use crate::types::scene::Scene;
use crate::traits::hittable::Hittable;
use crate::traits::hittable::HitRecord;
use crate::traits::sampleable::Sampleable;

pub struct Dielectric {
    pub refractive_index: f32,
}

impl Dielectric {
    pub fn new(refractive_index: f32) -> Self {
        Dielectric { refractive_index }
    }
}

fn dielectric_sample<'a>(dielectric: &Dielectric, rng: &mut rand::rngs::ThreadRng, hit_record: &HitRecord<'_>, scene: &'a Scene, depth: u32) -> vec::Vec3 {
    let reflected = vec::reflect(&vec::unit_vector(&hit_record.ray.direction), &hit_record.normal);
    let outward_normal;
    let ni_over_nt;
    let attenuation = vec::Vec3::new(1.0, 1.0, 1.0);
    let cosine;

    if hit_record.ray.direction.dot(&hit_record.normal) > 0.0 {
        // Ray is inside the material
        outward_normal = -hit_record.normal;
        ni_over_nt = dielectric.refractive_index;
        cosine = dielectric.refractive_index * hit_record.ray.direction.dot(&hit_record.normal) / hit_record.ray.direction.length();
    } else {
        // Ray is outside the material
        outward_normal = hit_record.normal;
        ni_over_nt = 1.0 / dielectric.refractive_index;
        cosine = -hit_record.ray.direction.dot(&hit_record.normal) / hit_record.ray.direction.length();
    }

    let refracted = vec::refract(&vec::unit_vector(&hit_record.ray.direction), &outward_normal, ni_over_nt);
    let scatter_direction = match refracted {
        Some(refracted) => {
            let r0 = ((1.0 - dielectric.refractive_index) / (1.0 + dielectric.refractive_index)).powi(2);
            let reflect_prob = r0 + (1.0 - r0) * (1.0 - cosine).powi(5);
            if rng.random::<f32>() < reflect_prob {
                reflected
            } else {
                refracted
            }
        },
        None => {
            // Total internal reflection
            reflected
        },
    };

    if depth == 0 {
        return vec::Vec3::new(0.0, 0.0, 0.0);
    }

    let scene_hit = scene.hit(&ray::Ray::new(&hit_record.point, &scatter_direction), 0.001, f32::MAX);
    if let Some(new_hit_record) = scene_hit {
        let bounce = new_hit_record.sampleable.sample(rng, &new_hit_record, scene, depth - 1);
        attenuation * bounce
    } else {
        vec::Vec3::new(0.0, 0.0, 0.0)
    }
}

impl Sampleable for Dielectric {
    fn sample(&self, rng: &mut rand::rngs::ThreadRng, hit_record: &HitRecord<'_>, scene: &Scene, depth: u32) -> vec::Vec3 {
        dielectric_sample(self, rng, hit_record, scene, depth)
    }
}

impl Sampleable for &Dielectric {
    fn sample(&self, rng: &mut rand::rngs::ThreadRng, hit_record: &HitRecord<'_>, scene: &Scene, depth: u32) -> vec::Vec3 {
        dielectric_sample(self, rng, hit_record, scene, depth)
    }
}
