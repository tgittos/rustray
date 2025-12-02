use crate::types::vec::Vec3;
use crate::types::vec;
use crate::traits::hittable::Hittable;
use crate::traits::hittable::HitRecord;
use crate::traits::sampleable::Sampleable;

pub struct Specular {
    pub albedo: Vec3,
}

impl Specular {
    pub fn new(albedo: &Vec3) -> Self {
        Specular { albedo: *albedo }
    }
}

fn specular_sample(specular: &Specular, rng: &mut rand::rngs::ThreadRng, hit_record: &HitRecord<'_>, scene: &Vec<Box<dyn Hittable>>, depth: u32) -> Vec3 {
    if depth == 0 {
        return Vec3::new(0.0, 0.0, 0.0);
    }

    let n = vec::unit_vector(&hit_record.normal);
    let reflected = hit_record.point + 2.0 * n.dot(&hit_record.point) * n;
    let new_ray = crate::Ray::new(&hit_record.point, &reflected);

    // bounce ray and attenuate
    for object in scene {
        if let Some(new_hit_record) = object.hit(&new_ray, 0.001, f32::MAX) {
            let bounce = new_hit_record.sampleable.sample(rng, &new_hit_record, scene, depth - 1);
            return specular.albedo * bounce;
        }
    }

    // miss
    Vec3::new(0.0, 0.0, 0.0)
}

impl Sampleable for Specular {
    fn sample(&self, rng: &mut rand::rngs::ThreadRng, hit_record: &HitRecord<'_>, scene: &Vec<Box<dyn Hittable>>, depth: u32) -> Vec3 {
        specular_sample(self, rng, hit_record, scene, depth)
    }
}

impl Sampleable for &Specular {
    fn sample(&self, rng: &mut rand::rngs::ThreadRng, hit_record: &HitRecord<'_>, scene: &Vec<Box<dyn Hittable>>, depth: u32) -> Vec3 {
        specular_sample(self, rng, hit_record, scene, depth)
    }
}
