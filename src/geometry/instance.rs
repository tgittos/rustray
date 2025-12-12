use std::sync::Arc;

use crate::core::bbox;
use crate::geometry::transform;
use crate::traits::hittable;

pub struct GeometryInstance {
    pub ref_obj: Arc<dyn hittable::Hittable>,
    pub transforms: Vec<transform::Transform>,
}

impl GeometryInstance {
    pub fn new(obj: Arc<dyn hittable::Hittable>) -> Self {
        Self {
            ref_obj: obj,
            transforms: Vec::new(),
        }
    }
}

impl hittable::Hittable for GeometryInstance {
    fn hit(&self, ray: &crate::core::ray::Ray, t_min: f32, t_max: f32) -> Option<hittable::Hit> {
        let mut mut_ray = ray.clone();
        // Apply inverse transforms to the ray here if needed.
        self.transforms.iter().rev().for_each(|transform| {
            mut_ray = transform.apply_inverse(&mut_ray);
        });

        let maybe_hit = self.ref_obj.hit(&mut_ray, t_min, t_max)?;

        let mut hit_point = maybe_hit.point;
        let mut normal = maybe_hit.normal;
        self.transforms.iter().for_each(|transform| {
            hit_point = transform.apply_point(&hit_point, ray.time);
            normal = transform.apply_normal(&normal, ray.time);
        });

        Some(hittable::Hit {
            ray: ray.clone(),
            t: maybe_hit.t,
            point: hit_point,
            normal,
            u: maybe_hit.u,
            v: maybe_hit.v,
        })
    }

    fn bounding_box(&self) -> bbox::BBox {
        self.transforms
            .iter()
            .fold(self.ref_obj.bounding_box(), |bbox, transform| {
                transform.apply_bbox(&bbox)
            })
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
