//! Lambertian diffuse material that scatters light uniformly.
use crate::math::{pdf::cosine, vec};
use crate::traits::scatterable::{ScatterRecord, Scatterable};
use crate::traits::{hittable, texturable};

/// Diffuse surface with a constant albedo.
pub struct Lambertian {
    pub texture: Box<dyn texturable::Texturable + Send + Sync>,
}

impl Lambertian {
    /// Creates a new diffuse material with the given albedo.
    pub fn new(texture: Box<dyn texturable::Texturable + Send + Sync>) -> Self {
        Self { texture }
    }
}

impl Scatterable for Lambertian {
    /// Provides a diffuse scatter record using cosine-weighted hemisphere sampling.
    fn scatter(
        &self,
        _rng: &mut rand::rngs::ThreadRng,
        hit_record: &hittable::HitRecord,
        depth: u32,
    ) -> Option<ScatterRecord> {
        if depth == 0 {
            return None;
        }

        Some(ScatterRecord {
            attenuation: self.texture.sample(&hit_record.hit),
            scatter_pdf: Some(Box::new(cosine::CosinePDF::new(&hit_record.hit.normal))),
            scattered_ray: None,
            use_light_pdf: true,
        })
    }

    fn emit(&self, _hit_record: &hittable::HitRecord) -> vec::Vec3 {
        vec::Vec3::new(0.0, 0.0, 0.0)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
