use crate::core::{vec, ray};
use crate::traits::hittable;

pub struct Sphere {
    pub center: vec::Vec3,
    pub radius: f32,
}

impl Sphere {
    pub fn new(center: &vec::Vec3, radius: f32) -> Self {
        Sphere {
            center: *center,
            radius,
        }
    }
}

impl hittable::Hittable for Sphere {
    fn hit(&self, ray: &ray::Ray, t_min: f32, t_max: f32) -> Option<hittable::Hit> {
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
                return Some(hittable::Hit { ray: ray.clone(), t: temp, point, normal });
            }
            let temp = (-b + discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                let point = ray.point_at(temp);
                let normal = (point - self.center) / self.radius;
                return Some(hittable::Hit { ray: ray.clone(), t: temp, point, normal });
            }
        }
        None
    }
}
