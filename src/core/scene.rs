//! Scene container that stores renderable objects and routes ray intersections.
use serde::{Deserialize, Serialize};
use std::{path::Path, time};

use crate::core::{bvh, ray, render, vec};
use crate::traits::{hittable, renderable};
use crate::utils::stats;

/// Collection of renderable objects making up the world.
#[derive(Serialize)]
pub struct Scene {
    pub objects: renderable::RenderableList,

    #[serde(skip)]
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
}

impl<'de> Deserialize<'de> for Scene {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let mut scene = Scene {
            objects: renderable::RenderableList::deserialize(deserializer)?,
            bvh: None,
        };

        let objects = std::mem::take(&mut scene.objects.objects);
        scene.bvh = Some(bvh::Bvh::new(&mut rand::rng(), objects));

        Ok(scene)
    }
}

#[typetag::serde]
impl renderable::Renderable for Scene {
    /// Finds the closest intersection among scene objects.
    fn hit(&self, ray: &ray::Ray, t_min: f32, t_max: f32) -> Option<hittable::HitRecord<'_>> {
        if let Some(bvh) = &self.bvh {
            return bvh.hit(ray, t_min, t_max);
        }

        let mut closest_so_far = t_max;
        let mut hit_record: Option<hittable::HitRecord> = None;

        if !self.objects.bbox.hit(ray, t_min, t_max) {
            return None;
        }

        for object in self.objects.objects.iter() {
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
            self.objects.bbox.clone()
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
}

pub fn load_from_file(
    _rng: &mut rand::rngs::ThreadRng,
    path: &Path,
) -> Result<render::Render, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(path)?;
    let render = toml::from_str(&content)?;
    Ok(render)
}
