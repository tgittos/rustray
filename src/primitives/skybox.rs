use crate::core::{vec, ray, scene};
use crate::traits::hittable;
use crate::traits::sampleable;
use crate::traits::renderable;

#[derive(Clone, Copy)]
pub struct Skybox {
    pub top_color: vec::Vec3,
    pub bottom_color: vec::Vec3,
}

impl Skybox {
    pub fn new(top_color: &vec::Vec3, bottom_color: &vec::Vec3) -> Self {
        Skybox {
            top_color: *top_color,
            bottom_color: *bottom_color,
        }
    }
}


fn skybox_hit(ray: &ray::Ray, _t_min: f32, t_max: f32) -> Option<hittable::Hit> {
    // Only act as a background; if we've already hit something closer, skip.
    if t_max < f32::MAX {
        return None;
    }
    let unit_direction = vec::unit_vector(&ray.direction);
    let t = 0.5 * (unit_direction.y + 1.0);
    let point = ray.point_at(1.0); // arbitrary point along the ray
    let normal = vec::Vec3::new(0.0, 0.0, 0.0); // normal is not used for skybox
    Some(hittable::Hit {
        ray: ray.clone(),
        t,
        point,
        normal,
    })
}

fn skybox_sample(
    _rng: &mut rand::rngs::ThreadRng,
    top_color: &vec::Vec3,
    bottom_color: &vec::Vec3,
    hit_record: &hittable::HitRecord,
    _scene: &scene::Scene,
    _depth: u32,
) -> vec::Vec3 {
    let unit_direction = vec::unit_vector(&hit_record.hit.point);
    let t = 0.5 * (unit_direction.y + 1.0);
    bottom_color * (1.0 - t) + top_color * t
}


impl hittable::Hittable for Skybox {
    fn hit(&self, ray: &ray::Ray, t_min: f32, t_max: f32) -> Option<hittable::Hit> {
        skybox_hit(ray, t_min, t_max)
    }
}

impl sampleable::Sampleable for Skybox {
    fn sample(
        &self,
        rng: &mut rand::rngs::ThreadRng,
        hit_record: &hittable::HitRecord,
        scene: &scene::Scene,
        depth: u32,
    ) -> vec::Vec3 {
        skybox_sample(rng, &self.top_color, &self.bottom_color, hit_record, scene, depth)
    }
}

impl renderable::Renderable for Skybox {
    fn hit(&self, ray: &ray::Ray, t_min: f32, t_max: f32) -> Option<hittable::HitRecord<'_>> {
        let maybe_hit = skybox_hit(ray, t_min, t_max);
        if maybe_hit.is_none() {
            return None;
        }

        let hit = maybe_hit.unwrap();
        let hit_record = hittable::HitRecord {
            hit: hit,
            renderable: self,
        };

        Some(hit_record)
    }

    fn sample(
        &self,
        rng: &mut rand::rngs::ThreadRng,
        hit_record: &hittable::HitRecord<'_>,
        scene: &scene::Scene,
        depth: u32,
    ) -> vec::Vec3 {
        skybox_sample(rng, &self.top_color, &self.bottom_color, hit_record, scene, depth)
    }
}