use std::sync::Arc;

use crate::math::vec;
use crate::traits::sampleable::Sampleable;

pub struct MaterialInstance {
    pub ref_mat: Arc<dyn Sampleable>,
    pub albedo: Option<vec::Vec3>,
}

impl MaterialInstance {
    pub fn new(mat: Arc<dyn Sampleable>) -> Self {
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

impl Sampleable for MaterialInstance {
    fn sample(
        &self,
        rng: &mut rand::rngs::ThreadRng,
        hit_record: &crate::traits::hittable::HitRecord,
        scene: &crate::core::scene::Scene,
        depth: u32,
    ) -> crate::math::vec::Vec3 {
        self.ref_mat.sample(rng, hit_record, scene, depth)
            * self.albedo.unwrap_or(vec::Vec3::new(1.0, 1.0, 1.0))
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
