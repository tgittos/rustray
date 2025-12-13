use rand::Rng;
use std::sync::Arc;

use crate::core::{bbox, ray, scene};
use crate::math::vec;
use crate::traits::renderable::Renderable;
use crate::traits::{hittable, renderable, sampleable, texturable};

pub struct Isotropic {
    pub texture: Box<dyn texturable::Texturable + Send + Sync>,
}

impl Isotropic {
    pub fn new(texture: Box<dyn texturable::Texturable + Send + Sync>) -> Self {
        Isotropic { texture }
    }
}

impl sampleable::Sampleable for Isotropic {
    fn sample(
        &self,
        rng: &mut rand::rngs::ThreadRng,
        hit_record: &hittable::HitRecord,
        scene: &scene::Scene,
        depth: u32,
    ) -> vec::Vec3 {
        if depth == 0 {
            return vec::Vec3::new(0.0, 0.0, 0.0);
        }

        let scattered = ray::Ray::new(
            &hit_record.hit.point,
            &vec::random_in_unit_sphere(rng),
            Some(hit_record.hit.ray.time),
        );

        if let Some(new_hit_record) = scene.hit(&scattered, 0.001, f32::MAX) {
            let bounce = new_hit_record
                .renderable
                .sample(rng, &new_hit_record, scene, depth - 1);

            return self.texture.sample(&hit_record.hit) * bounce;
        }

        vec::Vec3::new(0.0, 0.0, 0.0)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

pub struct RenderVolume {
    pub boundary: Box<dyn hittable::Hittable + Send + Sync>,
    pub density: f32,
    pub phase_function: Arc<dyn sampleable::Sampleable + Send + Sync>,
}

impl RenderVolume {
    pub fn new(
        boundary: Box<dyn hittable::Hittable + Send + Sync>,
        density: f32,
        phase_function: Arc<dyn sampleable::Sampleable + Send + Sync>,
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
            renderable: self,
        };

        Some(hit_record)
    }

    fn bounding_box(&self) -> bbox::BBox {
        self.boundary.bounding_box()
    }

    fn sample(
        &self,
        rng: &mut rand::rngs::ThreadRng,
        hit_record: &hittable::HitRecord,
        scene: &scene::Scene,
        depth: u32,
    ) -> vec::Vec3 {
        self.phase_function.sample(rng, hit_record, scene, depth)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
