//! Procedural sky gradient that acts as both geometry and material.
use serde::{Deserialize, Serialize};

use crate::core::{bbox, ray};
use crate::math::{pdf, vec};
use crate::traits::{hittable, renderable, scatterable};

#[derive(Clone, Copy, Serialize, Deserialize)]
/// Background gradient defined by top and bottom colors.
pub struct World {
    pub top_color: vec::Vec3,
    pub bottom_color: vec::Vec3,
}

impl World {
    /// Builds a skybox gradient.
    pub fn new(top_color: &vec::Vec3, bottom_color: &vec::Vec3) -> Self {
        World {
            top_color: *top_color,
            bottom_color: *bottom_color,
        }
    }
}

impl hittable::Hittable for World {
    /// Returns a dummy hit at infinity so the skybox can participate in rendering.
    fn hit(&self, ray: &ray::Ray, _t_min: f32, t_max: f32) -> Option<hittable::Hit> {
        // Only act as a background; if we've already hit something closer, skip.
        if t_max < f32::MAX {
            return None;
        }
        // Use a very large t so the BVH traversal doesn't prune foreground geometry.
        let t = f32::MAX;
        let point = ray.point_at(1.0); // arbitrary point along the ray
        let normal = vec::Vec3::new(0.0, 0.0, 0.0); // normal is not used for skybox
        Some(hittable::Hit {
            ray: ray.clone(),
            t,
            point,
            normal,
            u: 0.0,
            v: 0.0,
        })
    }

    fn bounding_box(&self) -> bbox::BBox {
        // Skybox is infinite; return a large bounding box.
        bbox::BBox::bounding(
            vec::Vec3::new(-f32::MAX, -f32::MAX, -f32::MAX),
            vec::Vec3::new(f32::MAX, f32::MAX, f32::MAX),
        )
    }

    fn get_pdf(&self, _origin: &vec::Point3, _time: f64) -> Box<dyn pdf::PDF + Send + Sync + '_> {
        Box::new(pdf::uniform::UniformPDF {})
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl scatterable::Scatterable for World {
    /// Emits a vertical gradient based on the ray direction.
    fn scatter(
        &self,
        _rng: &mut rand::rngs::ThreadRng,
        _hit_record: &hittable::HitRecord<'_>,
        _depth: u32,
    ) -> Option<scatterable::ScatterRecord> {
        None
    }

    fn emit(&self, hit_record: &hittable::HitRecord<'_>) -> vec::Vec3 {
        let unit_direction = vec::unit_vector(&hit_record.hit.ray.direction);
        let t = 0.5 * (unit_direction.y + 1.0);
        self.bottom_color * (1.0 - t) + self.top_color * t
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl renderable::Renderable for World {
    fn hit(&self, ray: &ray::Ray, t_min: f32, t_max: f32) -> Option<hittable::HitRecord<'_>> {
        let maybe_hit = (self as &dyn hittable::Hittable).hit(ray, t_min, t_max);

        if maybe_hit.is_none() {
            return None;
        }

        let hit = maybe_hit.unwrap();
        let hit_record = hittable::HitRecord {
            hit: hit,
            pdf: (self as &dyn hittable::Hittable).get_pdf(&hit.point, hit.ray.time),
            renderable: self,
        };

        Some(hit_record)
    }

    fn bounding_box(&self) -> bbox::BBox {
        // Skybox is infinite; return a large bounding box.
        bbox::BBox::bounding(
            vec::Vec3::new(-f32::MAX, -f32::MAX, -f32::MAX),
            vec::Vec3::new(f32::MAX, f32::MAX, f32::MAX),
        )
    }

    fn get_pdf(&self, origin: &vec::Point3, time: f64) -> Box<dyn pdf::PDF + Send + Sync + '_> {
        (self as &dyn hittable::Hittable).get_pdf(origin, time)
    }

    fn scatter(
        &self,
        rng: &mut rand::rngs::ThreadRng,
        hit_record: &hittable::HitRecord<'_>,
        depth: u32,
    ) -> Option<scatterable::ScatterRecord> {
        (self as &dyn scatterable::Scatterable).scatter(rng, hit_record, depth)
    }

    fn emit(&self, hit_record: &hittable::HitRecord<'_>) -> vec::Vec3 {
        (self as &dyn scatterable::Scatterable).emit(hit_record)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
