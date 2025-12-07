//! Basic sphere geometry implementing the `Hittable` trait.
use crate::core::{bbox, ray, vec};
use crate::traits::hittable;

#[derive(Debug, Clone)]
/// Sphere positioned at `center` with a `radius`.
pub struct Sphere {
    pub center: ray::Ray,
    pub radius: f32,
}

impl Sphere {
    /// Creates a new sphere; a negative radius flips the surface normal (useful for hollow spheres).
    pub fn new(center: &vec::Vec3, radius: f32) -> Self {
        Self {
            center: ray::Ray::new(center, &vec::Vec3::new(0.0, 0.0, 0.0), None),
            radius,
        }
    }

    pub fn moving(
        center_start: &vec::Vec3,
        center_end: &vec::Vec3,
        time_start: f64,
        time_end: f64,
        radius: f32,
    ) -> Self {
        Self {
            center: ray::Ray::new(
                center_start,
                &(*center_end - *center_start),
                Some(time_end - time_start),
            ),
            radius,
        }
    }
}

impl hittable::Hittable for Sphere {
    /// Solves the quadratic ray-sphere intersection and returns the nearest valid hit.
    fn hit(&self, ray: &ray::Ray, t_min: f32, t_max: f32) -> Option<hittable::Hit> {
        let current_center = if self.center.time == 0.0 {
            // Static sphere: keep the center fixed in time.
            self.center.origin
        } else {
            // Moving sphere: lerp the center based on the incoming ray time.
            self.center.origin + self.center.direction * (ray.time as f64 / self.center.time)
        };
        let oc = ray.origin - current_center;
        let a = ray.direction.dot(&ray.direction);
        let b = oc.dot(&ray.direction);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            for &sign in &[-1.0, 1.0] {
                let temp = (-b + sign * discriminant.sqrt()) / a;
                if temp < t_max && temp > t_min {
                    let point = ray.point_at(temp);
                    let normal = (point - current_center) / self.radius;
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

    fn bounding_box(&self) -> bbox::BBox {
        let radius_vec = vec::Vec3::new(self.radius, self.radius, self.radius);
        let t0_bb = bbox::BBox::bounding(
            self.center.point_at(0.0) - radius_vec,
            self.center.point_at(0.0) + radius_vec,
        );
        let t1_bb = bbox::BBox::bounding(
            self.center.point_at(1.0) - radius_vec,
            self.center.point_at(1.0) + radius_vec,
        );
        t0_bb.union(&t1_bb)
    }
}

impl hittable::Hittable for &Sphere {
    fn hit(&self, ray: &ray::Ray, t_min: f32, t_max: f32) -> Option<hittable::Hit> {
        (**self).hit(ray, t_min, t_max)
    }

    fn bounding_box(&self) -> crate::core::bbox::BBox {
        (**self).bounding_box()
    }
}
