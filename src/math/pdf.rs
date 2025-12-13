use crate::core::ray;
use crate::traits::hittable;

/// Probability Density Function trait
pub trait PDF {
    /// Sample a value from the PDF
    fn sample(
        &self,
        in_ray: &ray::Ray,
        hit_record: &hittable::HitRecord,
        out_ray: &ray::Ray,
    ) -> f32;
}
