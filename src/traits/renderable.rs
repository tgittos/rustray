use crate::core::{vec, ray, scene};
use crate::traits::hittable;
use crate::traits::sampleable;

pub trait Renderable {
    fn hit(&self, ray: &ray::Ray, t_min: f32, t_max: f32) -> Option<hittable::HitRecord<'_>>;
    fn sample(
        &self,
        rng: &mut rand::rngs::ThreadRng,
        hit_record: &hittable::HitRecord,
        scene: &scene::Scene,
        depth: u32,
    ) -> vec::Vec3;
}

pub struct RenderableImpl {
    pub hittable: Box<dyn hittable::Hittable>,
    pub sampleable: Box<dyn sampleable::Sampleable>,
}

impl Renderable for RenderableImpl {
    fn hit(&self, ray: &ray::Ray, t_min: f32, t_max: f32) -> Option<hittable::HitRecord<'_>> {
        let maybe_hit = self.hittable.hit(ray, t_min, t_max);
        if maybe_hit.is_none() {
            return None;
        }

        let hit = maybe_hit.unwrap();
        let hit_record = hittable::HitRecord {
            hit: hit,
            renderable: self,
        };

        Some(hit_record)
    }

    fn sample(
        &self,
        rng: &mut rand::rngs::ThreadRng,
        hit_record: &hittable::HitRecord<'_>,
        scene: &scene::Scene,
        depth: u32,
    ) -> vec::Vec3 {
        self.sampleable.sample(rng, hit_record, scene, depth)
    }
}

pub fn create_renderable(
    hittable: Box<dyn hittable::Hittable>,
    sampleable: Box<dyn sampleable::Sampleable>,
) -> RenderableImpl {
    RenderableImpl {
        hittable,
        sampleable,
    }
}