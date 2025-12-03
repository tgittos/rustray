use crate::vec;
use crate::ray;
use crate::traits::sampleable::Sampleable;

pub struct HitRecord<'a> {
    pub ray: ray::Ray,
    pub t: f32,
    pub point: vec::Vec3,
    pub normal: vec::Vec3,
    pub sampleable: &'a dyn Sampleable,
}

pub trait Hittable {
    fn hit(&self, ray: &ray::Ray, t_min: f32, t_max: f32) -> Option<HitRecord<'_>>;
}
