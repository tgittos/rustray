use crate::types::vec::Vec3;
use crate::traits::hittable::Hittable;
use crate::traits::hittable::HitRecord;

pub trait Sampleable {
    fn sample(&self, rng: &mut rand::rngs::ThreadRng, hit_record: &HitRecord<'_>, scene: &Vec<Box<dyn Hittable>>) -> Vec3;
}
