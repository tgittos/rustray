use crate::Vec3;
use crate::Ray;
use crate::types::vec;
use crate::traits::hittable::HitRecord;
use crate::traits::hittable::Hittable;
use crate::traits::sampleable::Sampleable;

pub struct Skybox {
    pub top_color: Vec3,
    pub bottom_color: Vec3,
}

impl Skybox {
    pub fn new(top_color: &Vec3, bottom_color: &Vec3) -> Self {
        Skybox {
            top_color: *top_color,
            bottom_color: *bottom_color,
        }
    }
}

fn skybox_sample(skybox: &Skybox, hit_record: &HitRecord) -> Vec3 {
    let unit_direction = vec::unit_vector(&hit_record.point);
    let t = 0.5 * (unit_direction.y + 1.0);
    skybox.bottom_color * (1.0 - t) + skybox.top_color * t
}

impl Sampleable for Skybox {
    fn sample(&self, _rng: &mut rand::rngs::ThreadRng, hit_record: &HitRecord<'_>, _scene: &Vec<Box<dyn Hittable>>) -> Vec3 {
        skybox_sample(self, hit_record)
    }
}

impl Sampleable for &Skybox {
    fn sample(&self, _rng: &mut rand::rngs::ThreadRng, hit_record: &HitRecord<'_>, _scene: &Vec<Box<dyn Hittable>>) -> Vec3 {
        skybox_sample(self, hit_record)
    }
}

impl Hittable for Skybox {
    fn hit(&self, ray: &Ray, _t_min: f32, t_max: f32) -> Option<HitRecord<'_>> {
        // Only act as a background; if we've already hit something closer, skip.
        if t_max < f32::MAX {
            return None;
        }
        let unit_direction = vec::unit_vector(&ray.direction);
        let t = 0.5 * (unit_direction.y + 1.0);
        let point = ray.point_at(1.0); // arbitrary point along the ray
        let normal = Vec3::new(0.0, 0.0, 0.0); // normal is not used for skybox
        Some(HitRecord {
            t,
            point,
            normal,
            sampleable: self,
        })
    }
}
