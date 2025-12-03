use crate::types::vec::Vec3;
use crate::types::scene::Scene;
use crate::traits::hittable::Hittable;
use crate::traits::hittable::HitRecord;

pub trait Sampleable {
    fn sample(
        &self,
        rng: &mut rand::rngs::ThreadRng,
        hit_record: &HitRecord<'_>,
        scene: &Scene,
        depth: u32,
    ) -> Vec3;
}
