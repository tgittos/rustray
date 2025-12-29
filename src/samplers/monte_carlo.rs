use crate::core::{camera, object, ray};
use crate::traits::hittable;
use crate::samplers::sampleable;

pub struct MonteCarloSampler<'a> {
    rng: rand::rngs::ThreadRng,
    spp: u32,
    spp_sqrt: u32,
    max_depth: u32,
    camera: &'a camera::Camera,
    objects: &'a object::Renderables,
    lights: &'a object::Renderables,
}

impl MonteCarloSampler<'_> {
    pub fn new(
        &mut rng: rand::rngs::ThreadRng,
        samples_per_pixel: u32,
        max_depth: u32,
        camera: &camera::Camera,
        objects: &object::Renderables,
        lights: &object::Renderables,
    ) -> Self {
        let (spp_sqrt, spp) = square_spp(samples_per_pixel);
        MonteCarloSampler {
            rng,
            spp,
            spp_sqrt,
            max_depth,
            camera,
            objects,
            lights,
        }
    }
}

impl sampleable::Sampleable for MonteCarloSampler<'_> {
    fn sample(&self) -> crate::math::vec::Vec3 {
        // stratified and jittered sampling
        for i in 0..self.spp_sqrt {
            for j in 0..self.spp_sqrt {
                let u = (i as f32 + self.rng.random::<f32>()) / self.spp_sqrt as f32;
                let v = (j as f32 + self.rng.random::<f32>()) / self.spp_sqrt as f32;

                // apply camera model to get ray
                let origin = self.camera.get_ray(&mut self.rng, u, v);
                let ray = ray::Ray::new(
                    &origin.origin,
                    &origin.direction,
                    Some(origin.time),
                );
            }
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

fn hit_objects<'a>(
    objects: &'a object::Renderables,
    ray: &'a ray::Ray,
    t_min: f32,
    t_max: f32,
) -> Option<hittable::Hit<'a>> {
    let mut closest_so_far = t_max;
    let mut hit_record: Option<hittable::Hit<'a>> = None;

    for obj in objects.iter() {
        if let Some(temp_hit) = obj.hit(ray, t_min, closest_so_far) {
            closest_so_far = temp_hit.t;
            hit_record = Some(temp_hit);
        }
    }

    hit_record
}

fn square_spp(spp: u32) -> (u32, u32) {
    let sqrt = (spp as f32).sqrt().ceil() as u32;
    (sqrt, sqrt * sqrt)
}