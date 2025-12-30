use rand::Rng;
use std::sync::Arc;

use crate::core::{bbox, ray};
use crate::math::{pdf, vec};
use crate::traits::{hittable, renderable, scatterable, texturable};

pub struct Isotropic {
    pub texture: Box<dyn texturable::Texturable + Send + Sync>,
    pub pdf: Box<dyn pdf::PDF + Send + Sync>,
}

impl Isotropic {
    pub fn new(texture: Box<dyn texturable::Texturable + Send + Sync>) -> Self {
        Self {
            texture,
            pdf: Box::new(pdf::phase::ConstantPhaseFunction {}),
        }
    }
}

impl scatterable::Scatterable for Isotropic {
    fn scatter(
        &self,
        _rng: &mut rand::rngs::ThreadRng,
        hit_record: &hittable::HitRecord,
        depth: u32,
    ) -> Option<scatterable::ScatterRecord> {
        if depth == 0 {
            return None;
        }

        Some(scatterable::ScatterRecord {
            attenuation: self.texture.sample(&hit_record.hit),
            scatter_pdf: Some(Box::new(pdf::phase::ConstantPhaseFunction {})),
            scattered_ray: None,
            use_light_pdf: false,
        })
    }

    fn emit(&self, _hit_record: &hittable::HitRecord) -> vec::Vec3 {
        vec::Vec3::new(0.0, 0.0, 0.0)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

pub struct RenderVolume {
    pub boundary: Box<dyn hittable::Hittable + Send + Sync>,
    pub density: f32,
    pub phase_function: Arc<dyn scatterable::Scatterable + Send + Sync>,
}

impl RenderVolume {
    pub fn new(
        boundary: Box<dyn hittable::Hittable + Send + Sync>,
        density: f32,
        phase_function: Arc<dyn scatterable::Scatterable + Send + Sync>,
    ) -> Self {
        RenderVolume {
            boundary,
            density,
            phase_function,
        }
    }
}

impl renderable::Renderable for RenderVolume {
    fn hit(&self, ray: &ray::Ray, t_min: f32, t_max: f32) -> Option<hittable::HitRecord<'_>> {
        // hit function to handle volumes
        let mut rec1 = self.boundary.hit(ray, f32::MIN, f32::MAX)?;
        let mut rec2 = self.boundary.hit(ray, rec1.t + 0.0001, f32::MAX)?;
        if rec1.t < t_min {
            rec1.t = t_min;
        }
        if rec2.t > t_max {
            rec2.t = t_max;
        }
        if rec1.t >= rec2.t {
            return None;
        }
        if rec1.t < 0.0 {
            rec1.t = 0.0;
        }

        let distance_inside_boundary = (rec2.t - rec1.t) * ray.direction.length();
        let hit_distance = -(1.0 / self.density) * rand::rng().random::<f32>().ln();
        if hit_distance > distance_inside_boundary {
            return None;
        }

        let t = rec1.t + hit_distance / ray.direction.length();
        let point = ray.point_at(t);
        let normal = vec::Vec3::new(1.0, 0.0, 0.0); // arbitrary
        let hit_record = hittable::HitRecord {
            hit: hittable::Hit {
                point,
                normal,
                t,
                ray: ray.clone(),
                u: 0.0,
                v: 0.0,
            },
            pdf: Box::new(pdf::phase::ConstantPhaseFunction {}),
            renderable: self,
        };

        Some(hit_record)
    }

    fn bounding_box(&self) -> bbox::BBox {
        self.boundary.bounding_box()
    }

    fn get_pdf(&self, _origin: &vec::Point3, _time: f64) -> Box<dyn pdf::PDF + Send + Sync + '_> {
        Box::new(pdf::phase::ConstantPhaseFunction {})
    }

    fn scatter(
        &self,
        rng: &mut rand::rngs::ThreadRng,
        hit_record: &hittable::HitRecord,
        depth: u32,
    ) -> Option<scatterable::ScatterRecord> {
        self.phase_function.scatter(rng, hit_record, depth)
    }

    fn emit(&self, hit_record: &hittable::HitRecord) -> vec::Vec3 {
        self.phase_function.emit(hit_record)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
