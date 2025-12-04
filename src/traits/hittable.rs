use crate::core::ray;
use crate::core::vec;
use crate::traits::renderable;

#[derive(Clone, Copy)]
pub struct Hit {
    pub ray: ray::Ray,
    pub t: f32,
    pub point: vec::Vec3,
    pub normal: vec::Vec3,
}

pub trait Hittable {
    fn hit(&self, ray: &ray::Ray, t_min: f32, t_max: f32) -> Option<Hit>;
}

pub struct HitRecord<'a> {
    pub hit: Hit,
    pub renderable: &'a dyn renderable::Renderable,
}

impl<'a> HitRecord<'a> {
    pub fn new<T: renderable::Renderable>(hit: Hit, renderable: &'a T) -> Self {
        HitRecord { hit, renderable }
    }
}
