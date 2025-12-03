use crate::core::vec;
use crate::core::ray;
use crate::types::scene::Scene;
use crate::traits::hittable::Hittable;
use crate::traits::hittable::HitRecord;
use crate::traits::sampleable::Sampleable;

pub struct Diffuse {
    pub albedo: vec::Vec3,
}

impl Diffuse {
    pub fn new(albedo: &vec::Vec3) -> Self {
        Diffuse {
            albedo: *albedo,
        }
    }
}

fn diffuse_sample(diffuse: &Diffuse, rng: &mut rand::rngs::ThreadRng, hit_record: &HitRecord<'_>, scene: &Scene, depth: u32) -> vec::Vec3 {
    if depth == 0 {
        return vec::Vec3::new(0.0, 0.0, 0.0);
    }

    let target = hit_record.point + hit_record.normal + vec::random_in_unit_sphere(rng);

    // bounce ray and attenuate
    let new_ray = ray::Ray::new(&hit_record.point, &(target - hit_record.point));
    if let Some(new_hit_record) = scene.hit(&new_ray, 0.001, f32::MAX) {
        let bounce = new_hit_record.sampleable.sample(rng, &new_hit_record, scene, depth - 1);
        return diffuse.albedo * (0.5 * bounce);
    }

    // miss
    vec::Vec3::new(0.0, 0.0, 0.0)
}

impl Sampleable for Diffuse {
    fn sample(&self, rng: &mut rand::rngs::ThreadRng, hit_record: &HitRecord<'_>, scene: &Scene, depth: u32) -> vec::Vec3 {
        diffuse_sample(self, rng, hit_record, scene, depth)
    }
}

impl Sampleable for &Diffuse {
    fn sample(&self, rng: &mut rand::rngs::ThreadRng, hit_record: &HitRecord<'_>, scene: &Scene, depth: u32) -> vec::Vec3 {
        diffuse_sample(self, rng, hit_record, scene, depth)
    }
}
