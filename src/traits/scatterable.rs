use std::any::Any;

use crate::core::ray;
use crate::math::{pdf, vec};
use crate::traits::hittable;

pub struct ScatterRecord {
    /// The color contribution from this scatter.
    pub attenuation: vec::Vec3,
    /// The PDF used for sampling or evaluating the scattered direction.
    pub scatter_pdf: Option<Box<dyn pdf::PDF + Send + Sync>>,
    /// The scattered ray for delta/specular events.
    pub scattered_ray: Option<ray::Ray>,
    /// Whether to sample from the scene-provided PDF (e.g. light mixing).
    pub use_light_pdf: bool,
}

pub trait Scatterable: Any + Send + Sync {
    fn scatter(
        &self,
        rng: &mut rand::rngs::ThreadRng,
        hit_record: &hittable::HitRecord,
        depth: u32,
    ) -> Option<ScatterRecord>;

    fn emit(&self, hit_record: &hittable::HitRecord) -> vec::Vec3;

    fn as_any(&self) -> &dyn Any;
}
