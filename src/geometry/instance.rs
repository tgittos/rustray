use std::sync::Arc;

use crate::core::{bbox, ray};
use crate::math::{pdf, vec};
use crate::geometry::transform;
use crate::traits::hittable;

pub struct GeometryInstance {
    pub ref_obj: Arc<dyn hittable::Hittable + Send + Sync>,
    pub transforms: Vec<transform::Transform>,
}

impl GeometryInstance {
    pub fn new(obj: Arc<dyn hittable::Hittable + Send + Sync>) -> Self {
        Self {
            ref_obj: obj,
            transforms: Vec::new(),
        }
    }
}

impl hittable::Hittable for GeometryInstance {
    fn hit(&self, ray: &ray::Ray, t_min: f32, t_max: f32) -> Option<hittable::Hit> {
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

    fn get_pdf(&self, origin: &vec::Point3, time: f64) -> Box<dyn pdf::PDF + Send + Sync + '_> {
        Box::new(GeometryInstancePDF::new(self, *origin, time))
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

struct GeometryInstancePDF<'a> {
    instance: &'a GeometryInstance,
    origin: vec::Point3,
    time: f64,
}

impl<'a> GeometryInstancePDF<'a> {
    fn new(instance: &'a GeometryInstance, origin: vec::Point3, time: f64) -> Self {
        Self {
            instance,
            origin,
            time,
        }
    }

    fn to_local(&self, point: &vec::Point3) -> vec::Point3 {
        let mut ray = ray::Ray::new(point, &vec::Vec3::new(0.0, 0.0, 0.0), Some(self.time));
        self.instance
            .transforms
            .iter()
            .rev()
            .for_each(|transform| {
                ray = transform.apply_inverse(&ray);
            });
        ray.origin
    }

    fn to_world(&self, point: &vec::Point3) -> vec::Point3 {
        let mut out = *point;
        self.instance.transforms.iter().for_each(|transform| {
            out = transform.apply_point(&out, self.time);
        });
        out
    }
}

impl pdf::PDF for GeometryInstancePDF<'_> {
    fn value(&self, direction: vec::Vec3) -> f32 {
        let local_origin = self.to_local(&self.origin);
        let world_point = self.origin + direction;
        let local_point = self.to_local(&world_point);
        let local_direction = local_point - local_origin;

        self.instance
            .ref_obj
            .get_pdf(&local_origin, self.time)
            .value(local_direction)
    }

    fn generate(&self, rng: &mut rand::rngs::ThreadRng) -> vec::Vec3 {
        let local_origin = self.to_local(&self.origin);
        let local_direction = self
            .instance
            .ref_obj
            .get_pdf(&local_origin, self.time)
            .generate(rng);
        let local_point = local_origin + local_direction;
        let world_point = self.to_world(&local_point);
        world_point - self.origin
    }
}
