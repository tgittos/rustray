use crate::traits::sampleable::Sampleable;
use crate::Vec3;
use crate::Ray;

pub struct HitRecord<'a> {
    pub t: f32,
    pub point: Vec3,
    pub normal: Vec3,
    pub sampleable: &'a dyn Sampleable,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord<'_>>;
}
