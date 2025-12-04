//! Basic sphere geometry implementing the `Hittable` trait.
use crate::core::{ray, vec};
use crate::traits::hittable;

#[derive(Debug, Clone)]
/// Sphere positioned at `center` with a `radius`.
pub struct Sphere {
    pub center: vec::Vec3,
    pub radius: f32,
}

impl Sphere {
    /// Creates a new sphere; a negative radius flips the surface normal (useful for hollow spheres).
    pub fn new(center: &vec::Vec3, radius: f32) -> Self {
        Self { center: *center, radius }
    }
}

impl hittable::Hittable for Sphere {
    /// Solves the quadratic ray-sphere intersection and returns the nearest valid hit.
    fn hit(&self, ray: &ray::Ray, t_min: f32, t_max: f32) -> Option<hittable::Hit> {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(&ray.direction);
        let b = oc.dot(&ray.direction);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            for &sign in &[-1.0, 1.0] {
                let temp = (-b + sign * discriminant.sqrt()) / a;
                if temp < t_max && temp > t_min {
                    let point = ray.point_at(temp);
                    let normal = (point - self.center) / self.radius;
                    return Some(hittable::Hit {
                        ray: ray.clone(),
                        t: temp,
                        point,
                        normal,
                    });
                }
            }
        }
        None
    }
}

impl hittable::Hittable for &Sphere {
    fn hit(&self, ray: &ray::Ray, t_min: f32, t_max: f32) -> Option<hittable::Hit> {
        (**self).hit(ray, t_min, t_max)
    }
}
