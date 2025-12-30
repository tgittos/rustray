use rand::rngs;

use crate::math::vec;
use crate::traits::scatterable::{ScatterRecord, Scatterable};
use crate::traits::{hittable, texturable};

pub struct DiffuseLight {
    pub texture: Box<dyn texturable::Texturable + Send + Sync>,
}

impl DiffuseLight {
    pub fn new(texture: Box<dyn texturable::Texturable + Send + Sync>) -> Self {
        DiffuseLight { texture }
    }
}

impl Scatterable for DiffuseLight {
    fn scatter(
        &self,
        _rng: &mut rngs::ThreadRng,
        _hit_record: &hittable::HitRecord,
        _depth: u32,
    ) -> Option<ScatterRecord> {
        None
    }

    fn emit(&self, hit_record: &hittable::HitRecord) -> vec::Vec3 {
        self.texture.sample(&hit_record.hit)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
