use rand::rngs;

use crate::core::{scene, vec};
use crate::traits::{hittable, sampleable, texturable};

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
        self.texture.sample(&hit_record.hit)
    }
}
