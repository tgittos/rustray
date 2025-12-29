//! Basic sphere geometry implementing the `Hittable` trait.
use serde::{Deserialize, Serialize};

use crate::core::{bbox, ray};
use crate::math::{pdf, vec};
use crate::traits::hittable;
use crate::traits::hittable::Hittable;

pub struct SpherePDF<'a> {
    sphere: &'a Sphere,
    origin: vec::Point3,
    time: f64,
}
impl pdf::PDF for SpherePDF<'_> {
    fn value(&self, direction: vec::Vec3) -> f32 {
        let ray = ray::Ray::new(&self.origin, &direction, Some(self.time));
        let Some(hit) = self.sphere.hit(&ray, 0.001, f32::MAX) else {
            return 0.0;
        };
        let area = 4.0 * std::f32::consts::PI * self.sphere.radius * self.sphere.radius;
        let direction_len_sq = direction.squared_length();
        if direction_len_sq <= f32::EPSILON {
            return 0.0;
        }
        let distance_squared = hit.t * hit.t * direction_len_sq;
        let cosine = (direction.dot(&hit.normal) / direction_len_sq.sqrt()).abs();
        if cosine <= 0.0 {
            return 0.0;
        }
        distance_squared / (cosine * area)
    }

    fn generate(&self, rng: &mut rand::rngs::ThreadRng) -> vec::Vec3 {
        let unit = vec::unit_vector(&vec::random_in_unit_sphere(rng));
        let point = self.sphere.center + unit * self.sphere.radius;
        point - self.origin
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Sphere positioned at `center` with a `radius`.
pub struct Sphere {
    pub center: vec::Vec3,
    pub radius: f32,
}

impl Sphere {
    /// Creates a new sphere; a negative radius flips the surface normal (useful for hollow spheres).
    pub fn new(center: &vec::Vec3, radius: f32) -> Self {
        Self {
            center: *center,
            radius,
        }
    }

    fn get_uv(p_unit: &vec::Vec3) -> (f32, f32) {
        // p_unit is expected to be the unit normal pointing outward from the sphere.
        let theta = (-p_unit.y).acos();
        let phi = -p_unit.z.atan2(p_unit.x) + std::f32::consts::PI;
        let u = phi / (2.0 * std::f32::consts::PI);
        let v = theta / std::f32::consts::PI;
        (u, v)
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
                    let (u, v) = Sphere::get_uv(&normal);
                    return Some(hittable::Hit {
                        ray: ray.clone(),
                        t: temp,
                        point,
                        normal,
                        u,
                        v,
                    });
                }
            }
        }
        None
    }

    fn bounding_box(&self) -> bbox::BBox {
        let radius_vec = vec::Vec3::new(self.radius, self.radius, self.radius);
        bbox::BBox::bounding(self.center - radius_vec, self.center + radius_vec)
    }

    fn get_pdf(&self, origin: &vec::Point3, time: f64) -> Box<dyn pdf::PDF + Send + Sync + '_> {
        Box::new(SpherePDF {
            sphere: self,
            origin: *origin,
            time,
        })
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
