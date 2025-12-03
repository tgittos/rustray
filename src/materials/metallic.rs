use crate::core::vec;
use crate::core::ray;

use crate::types::scene::Scene;
use crate::traits::hittable::Hittable;
use crate::traits::hittable::HitRecord;
use crate::traits::sampleable::Sampleable;

pub struct Metallic {
    pub albedo: vec::Vec3,
    pub roughness: f32,
}

impl Metallic {
    pub fn new(albedo: &vec::Vec3, roughness: f32) -> Self {
        Metallic { albedo: *albedo, roughness: if roughness < 1.0 { roughness } else { 1.0 } }
    }
}

fn metallic_sample<'a>(metallic: &Metallic, rng: &mut rand::rngs::ThreadRng, hit_record: &HitRecord<'_>, scene: &'a Scene, depth: u32) -> vec::Vec3 {
    if depth == 0 {
        return vec::Vec3::new(0.0, 0.0, 0.0);
    }

    let reflected = vec::reflect(&vec::unit_vector(&hit_record.ray.direction), &hit_record.normal);
    let scattered = ray::Ray::new(&hit_record.point, &(reflected + vec::random_in_unit_sphere(rng) * metallic.roughness));
    let mut closest_so_far = f32::MAX;
    let mut new_hit_record: Option<HitRecord<'a>> = None;

    if let Some(record) = scene.hit(&scattered, 0.001, closest_so_far) {
        closest_so_far = record.t;
        new_hit_record = Some(record);
    }

    if new_hit_record.is_none() {
        return vec::Vec3::new(0.0, 0.0, 0.0);
    }

    let new_hit_record = new_hit_record.unwrap();
    let bounce = new_hit_record.sampleable.sample(rng, &new_hit_record, scene, depth - 1);
    return metallic.albedo * bounce;
}

impl Sampleable for Metallic {
    fn sample(&self, rng: &mut rand::rngs::ThreadRng, hit_record: &HitRecord<'_>, scene: &Scene, depth: u32) -> vec::Vec3 {
        metallic_sample(self, rng, hit_record, scene, depth)
    }
}

impl Sampleable for &Metallic {
    fn sample(&self, rng: &mut rand::rngs::ThreadRng, hit_record: &HitRecord<'_>, scene: &Scene, depth: u32) -> vec::Vec3 {
        metallic_sample(self, rng, hit_record, scene, depth)
    }
}
