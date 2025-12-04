//! Scene container that stores renderable objects and routes ray intersections.
use crate::core::{ray, vec};
use crate::traits::hittable;
use crate::traits::renderable;

/// Collection of renderable objects making up the world.
pub struct Scene {
    pub objects: Vec<Box<dyn renderable::Renderable>>,
}

impl Scene {
    /// Creates an empty scene.
    pub fn new() -> Self {
        Scene {
            objects: Vec::new(),
        }
    }

    /// Adds a renderable object to the scene.
    pub fn add_object(&mut self, object: Box<dyn renderable::Renderable>) {
        self.objects.push(object);
    }
}

/// Finds the closest intersection among scene objects.
fn scene_hit<'a>(
    ray: &ray::Ray,
    objects: &'a Vec<Box<dyn renderable::Renderable + 'static>>,
    t_min: f32,
    t_max: f32,
) -> Option<hittable::HitRecord<'a>> {
    let mut closest_so_far = t_max;
    let mut hit_record: Option<hittable::HitRecord> = None;

    for object in objects {
        if let Some(temp_record) = object.hit(ray, t_min, closest_so_far) {
            closest_so_far = temp_record.hit.t;
            hit_record = Some(temp_record);
        }
    }

    hit_record
}

/// Delegates sampling to the material bound to the hit object.
fn scene_sample(
    rng: &mut rand::rngs::ThreadRng,
    hit_record: &hittable::HitRecord,
    scene: &Scene,
    depth: u32,
) -> vec::Vec3 {
    hit_record.renderable.sample(rng, hit_record, scene, depth)
}

impl renderable::Renderable for Scene {
    fn hit(&self, ray: &ray::Ray, t_min: f32, t_max: f32) -> Option<hittable::HitRecord<'_>> {
        scene_hit(ray, &self.objects, t_min, t_max)
    }

    fn sample(
        &self,
        rng: &mut rand::rngs::ThreadRng,
        hit_record: &hittable::HitRecord<'_>,
        scene: &Scene,
        depth: u32,
    ) -> vec::Vec3 {
        scene_sample(rng, hit_record, scene, depth)
    }
}

impl renderable::Renderable for &Scene {
    fn hit(&self, ray: &ray::Ray, t_min: f32, t_max: f32) -> Option<hittable::HitRecord<'_>> {
        scene_hit(ray, &self.objects, t_min, t_max)
    }

    fn sample(
        &self,
        rng: &mut rand::rngs::ThreadRng,
        hit_record: &hittable::HitRecord<'_>,
        scene: &Scene,
        depth: u32,
    ) -> vec::Vec3 {
        scene_sample(rng, hit_record, scene, depth)
    }
}
