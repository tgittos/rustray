use crate::Vec3;
use crate::Ray;
use crate::types::vec;
use crate::traits::hittable::Hittable;
use crate::traits::hittable::HitRecord;
use crate::traits::sampleable::Sampleable;

pub struct Diffuse {
    pub albedo: Vec3,
}

impl Diffuse {
    pub fn new(albedo: &Vec3) -> Self {
        Diffuse {
            albedo: *albedo,
        }
    }
}

fn diffuse_sample(diffuse: &Diffuse, rng: &mut rand::rngs::ThreadRng, hit_record: &HitRecord<'_>, scene: &Vec<Box<dyn Hittable>>) -> Vec3 {
    let target = hit_record.point + hit_record.normal + vec::random_in_unit_sphere(rng);

    // bounce ray and attenuate
    let new_ray = Ray::new(&hit_record.point, &(target - hit_record.point));
    for object in scene {
        if let Some(new_hit_record) = object.hit(&new_ray, 0.001, f32::MAX) {
            let bounce = new_hit_record.sampleable.sample(rng, &new_hit_record, scene);
            return diffuse.albedo * (0.5 * bounce);
        }
    }

    // miss
    Vec3::new(0.0, 0.0, 0.0)
}

impl Sampleable for Diffuse {
    fn sample(&self, rng: &mut rand::rngs::ThreadRng, hit_record: &HitRecord<'_>, scene: &Vec<Box<dyn Hittable>>) -> Vec3 {
        diffuse_sample(self, rng, hit_record, scene)
    }
}

impl Sampleable for &Diffuse {
    fn sample(&self, rng: &mut rand::rngs::ThreadRng, hit_record: &HitRecord<'_>, scene: &Vec<Box<dyn Hittable>>) -> Vec3 {
        diffuse_sample(self, rng, hit_record, scene)
    }
}
