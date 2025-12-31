use rand::Rng;

use crate::core::{camera, ray, scene};
use crate::math::vec;
use crate::samplers::sampleable::Sampleable;

pub type TraceRay =
    fn(&mut rand::rngs::ThreadRng, &scene::Scene, &ray::Ray, u32) -> vec::Vec3;

pub struct MonteCarloSampler<'a> {
    trace: TraceRay,
    spp: u32,
    spp_sqrt: u32,
    max_depth: u32,
    camera: &'a camera::Camera,
    scene: &'a scene::Scene,
}

impl<'a> MonteCarloSampler<'a> {
    pub fn new(
        samples_per_pixel: u32,
        max_depth: u32,
        camera: &'a camera::Camera,
        scene: &'a scene::Scene,
        trace: TraceRay,
    ) -> Self {
        let (spp_sqrt, spp) = square_spp(samples_per_pixel.max(1));
        MonteCarloSampler {
            trace,
            spp,
            spp_sqrt,
            max_depth,
            camera,
            scene,
        }
    }
}

impl Sampleable for MonteCarloSampler<'_> {
    fn sample_pixel(
        &self,
        rng: &mut rand::rngs::ThreadRng,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
    ) -> vec::Vec3 {
        let recip_spp_sqrt = 1.0 / self.spp_sqrt as f32;
        let recip_spp = 1.0 / self.spp as f32;
        let mut col = vec::Vec3::new(0.0, 0.0, 0.0);

        for i in 0..self.spp_sqrt {
            for j in 0..self.spp_sqrt {
                let u =
                    (x as f32 + (i as f32 + rng.random::<f32>()) * recip_spp_sqrt) / width as f32;
                let v = (y as f32 + (j as f32 + rng.random::<f32>()) * recip_spp_sqrt)
                    / height as f32;

                let r = self.camera.get_ray(rng, u, v);
                col = col + (self.trace)(rng, self.scene, &r, self.max_depth);
            }
        }

        col * recip_spp
    }
}

fn square_spp(spp: u32) -> (u32, u32) {
    let sqrt = (spp as f32).sqrt() as u32;
    (sqrt, sqrt * sqrt)
}
