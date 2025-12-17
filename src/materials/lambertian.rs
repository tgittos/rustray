//! Lambertian diffuse material that scatters light uniformly.
use std::time;

use crate::core::{ray, scene};
use crate::math::pdf::PDF;
use crate::math::{onb, pdf, vec};
use crate::stats::tracker;
use crate::traits::renderable::Renderable;
use crate::traits::{hittable, sampleable, texturable};

/// Diffuse surface with a constant albedo.
pub struct Lambertian {
    pub texture: Box<dyn texturable::Texturable + Send + Sync>,
}

struct CosinePDF {
    onb: onb::ONB,
}

impl CosinePDF {
    fn new(w: &vec::Vec3) -> Self {
        let onb = onb::ONB::build_from_w(w);
        Self { onb }
    }
}

impl pdf::PDF for CosinePDF {
    fn value(&self, direction: vec::Vec3) -> f32 {
        let cosine = vec::unit_vector(&direction).dot(&self.onb.w);
        if cosine <= 0.0 {
            0.0
        } else {
            cosine / std::f32::consts::PI
        }
    }

    fn generate(&self, rng: &mut rand::rngs::ThreadRng) -> vec::Vec3 {
        self.onb.local(&random_cosine_direction(rng))
    }
}

impl Lambertian {
    /// Creates a new diffuse material with the given albedo.
    pub fn new(texture: Box<dyn texturable::Texturable + Send + Sync>) -> Self {
        Self { texture }
    }
}

fn random_cosine_direction(rng: &mut rand::rngs::ThreadRng) -> vec::Vec3 {
    let r1: f32 = rand::Rng::random::<f32>(rng);
    let r2: f32 = rand::Rng::random::<f32>(rng);
    let z = (1.0 - r2).sqrt();

    let phi = 2.0 * std::f32::consts::PI * r1;
    let x = phi.cos() * r2.sqrt();
    let y = phi.sin() * r2.sqrt();

    vec::Vec3::new(x, y, z)
}

impl sampleable::Sampleable for Lambertian {
    /// Samples a diffuse bounce using cosine-weighted hemisphere sampling.
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

        let sample_start = time::Instant::now();

        // bounce ray and attenuate
        let pdf = CosinePDF::new(&hit_record.hit.normal);
        let scattered_ray = ray::Ray::new(
            &hit_record.hit.point,
            &pdf.generate(rng),
            Some(hit_record.hit.ray.time),
        );
        let hit_start = time::Instant::now();
        let maybe_hit = scene.hit(&scattered_ray, 0.001, f32::MAX);
        let hit_elapsed = hit_start.elapsed();
        tracker::add_hit_stat(tracker::Stat::new(tracker::LAMBERTIAN_HIT, hit_elapsed));

        if let Some(new_hit_record) = maybe_hit {
            let bounce_start = time::Instant::now();
            let bounce = new_hit_record
                .renderable
                .sample(rng, &new_hit_record, scene, depth - 1);
            let bounce_elapsed = bounce_start.elapsed();
            tracker::add_sample_stat(tracker::Stat::new(
                tracker::LAMBERTIAN_SAMPLE,
                sample_start
                    .elapsed()
                    .saturating_sub(hit_elapsed + bounce_elapsed),
            ));

            let attenuation = self.texture.sample(&hit_record.hit);
            let pdf = pdf.value(scattered_ray.direction);

            return (attenuation * bounce * pdf) / pdf;
        }
        tracker::add_sample_stat(tracker::Stat::new(
            tracker::LAMBERTIAN_SAMPLE,
            sample_start.elapsed().saturating_sub(hit_elapsed),
        ));

        // miss
        vec::Vec3::new(0.0, 0.0, 0.0)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
