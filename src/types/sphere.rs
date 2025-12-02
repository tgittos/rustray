use crate::Vec3;
use crate::Ray;
use crate::traits::hittable::Hittable;
use crate::traits::sampleable::Sampleable;
use crate::traits::hittable::HitRecord;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    material: Box<dyn Sampleable>,
}

impl Sphere {
    pub fn new(center: &Vec3, radius: f32, material: Option<Box<dyn Sampleable>>) -> Self {
        Sphere {
            center: *center,
            radius,
            material: material.unwrap_or(Box::new(crate::materials::diffuse::Diffuse::new(&Vec3::new(0.5, 0.5, 0.5)))),
        }
    }
}

impl Sampleable for Sphere {
    fn sample(&self, rng: &mut rand::rngs::ThreadRng, hit_record: &HitRecord<'_>, scene: &Vec<Box<dyn Hittable>>) -> Vec3 {
        self.material.sample(rng, hit_record, scene)
    }
}

impl Sampleable for &Sphere {
    fn sample(&self, rng: &mut rand::rngs::ThreadRng, hit_record: &HitRecord<'_>, scene: &Vec<Box<dyn Hittable>>) -> Vec3 {
        self.material.sample(rng, hit_record, scene)
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord<'_>> {
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
                return Some(HitRecord { t: temp, point, normal, sampleable: self });
            }
            let temp = (-b + discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                let point = ray.point_at(temp);
                let normal = (point - self.center) / self.radius;
                return Some(HitRecord { t: temp, point, normal, sampleable: self });
            }
        }
        None
    }
}
