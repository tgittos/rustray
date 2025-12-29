use std::any::Any;

use crate::core::ray;
use crate::math::{vec, pdf};
use crate::traits::hittable;

pub struct ScatterRecord<'a> {
    /// The color contribution from this scatter.
    pub attenuation: vec::Vec3,
    /// The PDF used for sampling the scattered direction.
    pub scatter_pdf: &'a dyn pdf::PDF,
    /// The scattered ray to use if ignoring the PDF sampling.
    pub scattered_ray: Option<ray::Ray>,
}

pub trait Scatterable {
    fn scatter(
        &self,
        rng: &mut rand::rngs::ThreadRng,
        hit: &hittable::Hit,
        depth: u32,
    ) -> Option<ScatterRecord<'_>>;

    fn emit(
        &self,
        rng: &mut rand::rngs::ThreadRng,
        scatter_record: &ScatterRecord,
    ) -> vec::Vec3;

    fn as_any(&self) -> &dyn Any;
}