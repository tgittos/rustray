use std::sync::Arc;

use crate::math::vec;
use crate::traits::scatterable::{ScatterRecord, Scatterable};

pub struct MaterialInstance {
    pub ref_mat: Arc<dyn Scatterable + Send + Sync>,
    pub albedo: Option<vec::Vec3>,
}

impl MaterialInstance {
    pub fn new(mat: Arc<dyn Scatterable + Send + Sync>) -> Self {
        Self {
            ref_mat: mat,
            albedo: None,
        }
    }

    pub fn with_albedo(mut self, albedo: vec::Vec3) -> Self {
        self.albedo = Some(albedo);
        self
    }
}

impl Scatterable for MaterialInstance {
    fn scatter(
        &self,
        rng: &mut rand::rngs::ThreadRng,
        hit_record: &crate::traits::hittable::HitRecord,
        depth: u32,
    ) -> Option<ScatterRecord> {
        let mut scatter_record = self.ref_mat.scatter(rng, hit_record, depth)?;
        let tint = self.albedo.unwrap_or(vec::Vec3::new(1.0, 1.0, 1.0));
        scatter_record.attenuation = scatter_record.attenuation * tint;
        Some(scatter_record)
    }

    fn emit(&self, hit_record: &crate::traits::hittable::HitRecord) -> vec::Vec3 {
        self.ref_mat.emit(hit_record) * self.albedo.unwrap_or(vec::Vec3::new(1.0, 1.0, 1.0))
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
