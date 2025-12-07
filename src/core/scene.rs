//! Scene container that stores renderable objects and routes ray intersections.
use std::time;

use crate::core::{bvh, ray, vec};
use crate::traits::hittable;
use crate::traits::renderable;
use crate::traits::renderable::Renderable;
use crate::utils::stats;

/// Collection of renderable objects making up the world.
pub struct Scene {
    pub objects: renderable::RenderableList,
    pub bvh: Option<bvh::Bvh>,
}

impl Scene {
    /// Creates an empty scene.
    pub fn new() -> Self {
        Scene {
            objects: renderable::RenderableList::new(),
            bvh: None,
        }
    }

    /// Adds a renderable object to the scene.
    pub fn add_object(&mut self, object: Box<dyn renderable::Renderable>) {
        if let Some(bvh) = self.bvh.take() {
            // BVH already exists; flatten it back into the list before adding more.
            let renderables = bvh.into_renderables();
            self.objects = renderable::RenderableList::new();
            renderables.into_iter().for_each(|renderable| {
                self.objects.add(renderable);
            });
        }
        self.objects.add(object);
    }

    /// Builds the BVH from all current renderables.
    pub fn build_bvh(&mut self, rng: &mut rand::rngs::ThreadRng) {
        if self.objects.objects.is_empty() {
            self.bvh = None;
            return;
        }

        let renderables = std::mem::take(&mut self.objects.objects);
        let bvh_root = bvh::Bvh::new(rng, renderables);
        self.objects.bbox = bvh_root.bounding_box().clone();
        self.bvh = Some(bvh_root);
    }
}

/// Finds the closest intersection among scene objects.
fn scene_hit<'a>(
    ray: &ray::Ray,
    scene: &'a Scene,
    t_min: f32,
    t_max: f32,
) -> Option<hittable::HitRecord<'a>> {
    if let Some(bvh) = &scene.bvh {
        return bvh.hit(ray, t_min, t_max);
    }

    let mut closest_so_far = t_max;
    let mut hit_record: Option<hittable::HitRecord> = None;

    if !scene.objects.bbox.hit(ray, t_min, t_max) {
        return None;
    }

    for object in scene.objects.objects.iter() {
        let hit_start = time::Instant::now();
        if let Some(temp_record) = object.hit(ray, t_min, closest_so_far) {
            // stats::add_hit_stat(stats::Stat::new(
            //     stats::SCENE_HIT, hit_start.elapsed()
            // ));

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
    let sample_start = time::Instant::now();
    let result = hit_record.renderable.sample(rng, hit_record, scene, depth);
    // stats::add_sample_stat(stats::Stat::new(
    //     stats::SCENE_SAMPLE, sample_start.elapsed()
    // ));
    result
}

impl renderable::Renderable for Scene {
    fn hit(&self, ray: &ray::Ray, t_min: f32, t_max: f32) -> Option<hittable::HitRecord<'_>> {
        scene_hit(ray, self, t_min, t_max)
    }

    fn bounding_box(&self) -> super::bbox::BBox {
        if let Some(bvh) = &self.bvh {
            bvh.bounding_box().clone()
        } else {
            self.objects.bbox.clone()
        }
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
        scene_hit(ray, self, t_min, t_max)
    }

    fn bounding_box(&self) -> super::bbox::BBox {
        if let Some(bvh) = &self.bvh {
            bvh.bounding_box().clone()
        } else {
            self.objects.bbox.clone()
        }
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
