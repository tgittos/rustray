use crate::core::{scene, vec};
use crate::traits::hittable;

pub trait Sampleable {
    fn sample(
        &self,
        rng: &mut rand::rngs::ThreadRng,
        hit_record: &hittable::HitRecord,
        scene: &scene::Scene,
        depth: u32,
    ) -> vec::Vec3;
}
