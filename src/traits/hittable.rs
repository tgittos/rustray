//! Abstractions for geometry that can be intersected by rays.
use crate::core::{bbox, ray, vec};
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
}

/// Trait for objects that can be intersected by rays.
pub trait Hittable {
    /// Determines if a ray hits the object within the given t range.
    /// Bounds intersection tests between t_min and t_max.
    /// Returns Some([`Hit`]) if there is a hit, otherwise None.
    fn hit(&self, ray: &ray::Ray, t_min: f32, t_max: f32) -> Option<Hit>;

    /// Returns the bounding box of the object.
    fn bounding_box(&self) -> bbox::BBox;
}

/// A record of a hit, associating the hit information with the renderable object.
pub struct HitRecord<'a> {
    pub hit: Hit,
    pub renderable: &'a dyn renderable::Renderable,
}

impl<'a> HitRecord<'a> {
    /// Creates a new HitRecord.
    pub fn new<T: renderable::Renderable>(hit: Hit, renderable: &'a T) -> Self {
        HitRecord { hit, renderable }
    }
}
