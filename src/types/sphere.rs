use crate::core::vec;
use crate::core::ray;
use crate::types::scene::Scene;
use crate::traits::hittable::Hittable;
use crate::traits::sampleable::Sampleable;
use crate::traits::hittable::HitRecord;

pub struct Sphere {
    pub center: vec::Vec3,
    pub radius: f32,
    material: Box<dyn Sampleable>,
}

impl Sphere {
    pub fn new(center: &vec::Vec3, radius: f32, material: Option<Box<dyn Sampleable>>) -> Self {
        Sphere {
            center: *center,
            radius,
            material: material.unwrap_or(Box::new(crate::materials::diffuse::Diffuse::new(&vec::Vec3::new(0.5, 0.5, 0.5)))),
        }
    }
}

impl Sampleable for Sphere {
    fn sample(&self, rng: &mut rand::rngs::ThreadRng, hit_record: &HitRecord<'_>, scene: &Scene, depth: u32) -> vec::Vec3 {
        self.material.sample(rng, hit_record, scene, depth)
    }
}

impl Sampleable for &Sphere {
    fn sample(&self, rng: &mut rand::rngs::ThreadRng, hit_record: &HitRecord<'_>, scene: &Scene, depth: u32) -> vec::Vec3 {
        self.material.sample(rng, hit_record, scene, depth)
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &ray::Ray, t_min: f32, t_max: f32) -> Option<HitRecord<'_>> {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(&ray.direction);
        let b = oc.dot(&ray.direction);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let temp = (-b - discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                let point = ray.point_at(temp);
                let normal = (point - self.center) / self.radius;
                return Some(HitRecord { ray: ray.clone(), t: temp, point, normal, sampleable: self });
            }
            let temp = (-b + discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                let point = ray.point_at(temp);
                let normal = (point - self.center) / self.radius;
                return Some(HitRecord { ray: ray.clone(), t: temp, point, normal, sampleable: self });
            }
        }
        None
    }
}
