//! Scene container that stores renderable objects and routes ray intersections.
use std::{path::Path, time};

use crate::core::{bvh, object, ray, render};
use crate::math::vec;
use crate::stats;
use crate::traits::{hittable, renderable};

/// Collection of renderable objects making up the world.
pub struct Scene {
    pub renderables: object::Renderables,

    pub bvh: Option<bvh::Bvh>,
}

impl Scene {
    /// Creates an empty scene.
    pub fn new() -> Self {
        Scene {
            renderables: object::Renderables::new(),
            bvh: None,
        }
    }

    /// Adds a renderable object to the scene.
    pub fn add_object(&mut self, object: Box<dyn renderable::Renderable>) {
        self.renderables.add(object);
    }

    pub fn build_bvh(&mut self, rng: &mut rand::rngs::ThreadRng) {
        if self.renderables.objects.is_empty() {
            self.bvh = None;
            return;
        }
        self.renderables.rebuild_bbox();
        self.bvh = Some(bvh::Bvh::new(rng, &self.renderables.objects));
    }
}

impl renderable::Renderable for Scene {
    /// Finds the closest intersection among scene objects.
    fn hit(&self, ray: &ray::Ray, t_min: f32, t_max: f32) -> Option<hittable::HitRecord<'_>> {
        if let Some(bvh) = &self.bvh {
            return bvh.hit(&self.renderables.objects, ray, t_min, t_max);
        }

        let mut closest_so_far = t_max;
        let mut hit_record: Option<hittable::HitRecord> = None;

        if !self.renderables.bbox.hit(ray, t_min, t_max) {
            return None;
        }

        for object in self.renderables.objects.iter() {
            let hit_start = time::Instant::now();
            if let Some(temp_record) = object.hit(ray, t_min, closest_so_far) {
                stats::add_hit_stat(stats::Stat::new(stats::SCENE_HIT, hit_start.elapsed()));

                closest_so_far = temp_record.hit.t;
                hit_record = Some(temp_record);
            }
        }

        hit_record
    }

    /// Returns the bounding box of the scene, which is either the BVH's bounding box
    fn bounding_box(&self) -> super::bbox::BBox {
        if let Some(bvh) = &self.bvh {
            bvh.bounding_box().clone()
        } else {
            self.renderables.bbox.clone()
        }
    }

    /// Delegates sampling to the material bound to the hit object.
    fn sample(
        &self,
        rng: &mut rand::rngs::ThreadRng,
        hit_record: &hittable::HitRecord<'_>,
        scene: &Scene,
        depth: u32,
    ) -> vec::Vec3 {
        let sample_start = time::Instant::now();
        let result = hit_record.renderable.sample(rng, hit_record, scene, depth);
        stats::add_sample_stat(stats::Stat::new(
            stats::SCENE_SAMPLE,
            sample_start.elapsed(),
        ));
        result
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

pub fn load_from_file(
    rng: &mut rand::rngs::ThreadRng,
    path: &Path,
) -> Result<render::Render, Box<dyn std::error::Error>> {
    crate::core::scene_file::load_render(rng, path).map_err(|e| e.into())
}
