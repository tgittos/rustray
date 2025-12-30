//! Scene container that stores renderable objects and routes ray intersections.
use std::path::Path;

use crate::core::{bvh, object, ray, render};
use crate::math::{pdf, vec};
use crate::traits::{hittable, renderable, scatterable};

/// Collection of renderable objects making up the world.
pub struct Scene {
    pub renderables: object::Renderables,
    pub lights: Vec<Box<dyn renderable::Renderable + Send + Sync>>,

    pub bvh: Option<bvh::Bvh>,
}

impl Scene {
    /// Creates an empty scene.
    pub fn new() -> Self {
        Scene {
            renderables: object::Renderables::new(),
            lights: Vec::new(),
            bvh: None,
        }
    }

    /// Adds a renderable object to the scene.
    pub fn add_object(&mut self, object: Box<dyn renderable::Renderable + Send + Sync>) {
        self.renderables.add(object);
    }

    pub fn add_light(&mut self, light: Box<dyn renderable::Renderable + Send + Sync>) {
        self.lights.push(light);
    }

    pub fn build_bvh(&mut self, rng: &mut rand::rngs::ThreadRng) {
        if self.renderables.objects.is_empty() {
            self.bvh = None;
            return;
        }
        self.renderables.rebuild_bbox();
        self.bvh = Some(bvh::Bvh::new(rng, &self.renderables.objects));
    }

    fn apply_light_pdf<'a>(
        &'a self,
        hit_record: hittable::HitRecord<'a>,
    ) -> hittable::HitRecord<'a> {
        let mut mixed_pdf = pdf::MixturePDF::new();
        if hit_record.hit.normal.squared_length() <= f32::EPSILON {
            mixed_pdf.add(Box::new(pdf::uniform::UniformPDF {}), 1.0);
            return hittable::HitRecord {
                hit: hit_record.hit,
                pdf: Box::new(mixed_pdf),
                renderable: hit_record.renderable,
            };
        }

        let cosine_pdf = pdf::cosine::CosinePDF::new(&hit_record.hit.normal);
        if self.lights.is_empty() {
            mixed_pdf.add(Box::new(cosine_pdf), 1.0);
        } else {
            mixed_pdf.add(Box::new(cosine_pdf), 0.5);
            let light_weight = 0.5 / self.lights.len() as f32;
            for light in self.lights.iter() {
                mixed_pdf.add(
                    light.get_pdf(&hit_record.hit.point, hit_record.hit.ray.time),
                    light_weight,
                );
            }
        }

        hittable::HitRecord {
            hit: hit_record.hit,
            pdf: Box::new(mixed_pdf),
            renderable: hit_record.renderable,
        }
    }
}

impl renderable::Renderable for Scene {
    /// Finds the closest intersection among scene objects.
    fn hit(&self, ray: &ray::Ray, t_min: f32, t_max: f32) -> Option<hittable::HitRecord<'_>> {
        if let Some(bvh) = &self.bvh {
            return bvh
                .hit(&self.renderables.objects, ray, t_min, t_max)
                .map(|record| self.apply_light_pdf(record));
        }

        let mut closest_so_far = t_max;
        let mut hit_record: Option<hittable::HitRecord> = None;

        if !self.renderables.bbox.hit(ray, t_min, t_max) {
            return None;
        }

        for object in self.renderables.objects.iter() {
            if let Some(temp_record) = object.hit(ray, t_min, closest_so_far) {
                closest_so_far = temp_record.hit.t;
                hit_record = Some(temp_record);
            }
        }

        hit_record.map(|record| self.apply_light_pdf(record))
    }

    /// Returns the bounding box of the scene, which is either the BVH's bounding box
    /// or the combined bounding box of all renderables.
    fn bounding_box(&self) -> super::bbox::BBox {
        if let Some(bvh) = &self.bvh {
            bvh.bounding_box().clone()
        } else {
            self.renderables.bbox.clone()
        }
    }

    fn get_pdf(&self, _origin: &vec::Point3, _time: f64) -> Box<dyn pdf::PDF + Send + Sync + '_> {
        Box::new(pdf::uniform::UniformPDF {})
    }

    /// Delegates scattering to the material bound to the hit object.
    fn scatter(
        &self,
        rng: &mut rand::rngs::ThreadRng,
        hit_record: &hittable::HitRecord<'_>,
        depth: u32,
    ) -> Option<scatterable::ScatterRecord> {
        let result = hit_record.renderable.scatter(rng, hit_record, depth);
        result
    }

    fn emit(&self, hit_record: &hittable::HitRecord<'_>) -> vec::Vec3 {
        hit_record.renderable.emit(hit_record)
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
