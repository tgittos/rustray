//! Abstractions for geometry that can be intersected by rays.
use std::any::Any;

use crate::core::{bbox, ray};
use crate::math::{pdf, vec};
use crate::traits::renderable;

/// Information about a ray-object intersection.
#[derive(Clone, Copy)]
pub struct Hit {
    /// Ray that produced the hit.
    pub ray: ray::Ray,
    /// Parameter along the ray where the hit occurred.
    pub t: f32,
    /// World-space hit position.
    pub point: vec::Vec3,
    /// Surface normal pointing outward from the hit.
    pub normal: vec::Vec3,
    /// Texture coordinates at the hit point.
    pub u: f32,
    /// Texture coordinates at the hit point.
    pub v: f32,
}

/// Trait for objects that can be intersected by rays.
pub trait Hittable: Any + Send + Sync {
    /// Determines if a ray hits the object within the given t range.
    /// Bounds intersection tests between t_min and t_max.
    /// Returns Some([`Hit`]) if there is a hit, otherwise None.
    fn hit(&self, ray: &ray::Ray, t_min: f32, t_max: f32) -> Option<Hit>;

    /// Returns the bounding box of the object.
    fn bounding_box(&self) -> bbox::BBox;

    /// Returns a probability density function for sampling directions toward the object.
    fn get_pdf(&self, origin: &vec::Point3, time: f64) -> Box<dyn pdf::PDF + Send + Sync + '_>;

    /// Allows downcasting to concrete types.
    fn as_any(&self) -> &dyn Any;
}

/// A record of a hit, associating the hit information with the renderable object.
pub struct HitRecord<'a> {
    pub hit: Hit,
    pub pdf: Box<dyn pdf::PDF + Send + Sync + 'a>,
    pub renderable: &'a dyn renderable::Renderable,
}

impl<'a> HitRecord<'a> {
    /// Creates a new HitRecord.
    pub fn new(
        hit: Hit,
        pdf: Box<dyn pdf::PDF + Send + Sync + 'a>,
        renderable: &'a dyn renderable::Renderable,
    ) -> Self {
        HitRecord {
            hit,
            pdf,
            renderable,
        }
    }
}
