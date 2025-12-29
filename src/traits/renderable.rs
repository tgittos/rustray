//! Glue trait combining geometry (hittable) and material sampling.
use std::any::Any;

use crate::core::{bbox, ray, scene};
use crate::math::{pdf, vec};
use crate::traits::hittable;

/// Trait for objects that can be rendered in the scene.
pub trait Renderable: Any + Send + Sync {
    /// Determines if a ray hits the renderable object within the given t range.
    /// Returns [`hittable::HitRecord`] Some(HitRecord) if there is a hit, otherwise None.
    ///
    /// # Arguments
    /// * [`ray::Ray`] `ray` - The ray to test for intersection.
    /// * `t_min` - The minimum t value for valid intersections.
    /// * `t_max` - The maximum t value for valid intersections.
    ///
    /// # Returns
    /// An Option containing a [`hittable::HitRecord`] HitRecord if the ray hits the object, otherwise None.
    fn hit(&self, ray: &ray::Ray, t_min: f32, t_max: f32) -> Option<hittable::HitRecord<'_>>;

    /// Returns the bounding box of the renderable object.
    fn bounding_box(&self) -> bbox::BBox;

    /// Returns a probability density function for sampling directions toward the renderable object.
    fn get_pdf(
        &self,
        origin: &vec::Point3,
        time: f64,
    ) -> Box<dyn pdf::PDF + Send + Sync + '_>;

    /// Samples the color contribution at the hit point.
    fn sample(
        &self,
        rng: &mut rand::rngs::ThreadRng,
        hit_record: &hittable::HitRecord,
        scene: &scene::Scene,
        depth: u32,
    ) -> vec::Vec3;

    fn as_any(&self) -> &dyn Any;
}
