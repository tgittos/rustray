use rand::rngs;
use std::time;

use crate::core::scene;
use crate::math::vec;
use crate::traits::{hittable, sampleable, texturable};
use crate::stats;

pub struct DiffuseLight {
    pub texture: Box<dyn texturable::Texturable>,
}

impl DiffuseLight {
    pub fn new(texture: Box<dyn texturable::Texturable>) -> Self {
        DiffuseLight { texture }
    }
}

impl sampleable::Sampleable for DiffuseLight {
    fn sample(
        &self,
        _rng: &mut rngs::ThreadRng,
        hit_record: &hittable::HitRecord,
        _scene: &scene::Scene,
        _depth: u32,
    ) -> vec::Vec3 {
        let sample_start = time::Instant::now();
        let result = self.texture.sample(&hit_record.hit);
        stats::add_sample_stat(stats::Stat::new(
            stats::DIFFUSE_LIGHT_SAMPLE,
            sample_start.elapsed(),
        ));
        result
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
