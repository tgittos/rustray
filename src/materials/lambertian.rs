//! Lambertian diffuse material that scatters light uniformly.
use std::time;

use crate::core::{ray, scene};
use crate::math::{pdf::cosine, vec};
use crate::math::pdf::PDF;
use crate::stats::tracker;
use crate::traits::renderable::Renderable;
use crate::traits::{hittable, sampleable, texturable};

/// Diffuse surface with a constant albedo.
pub struct Lambertian {
    pub texture: Box<dyn texturable::Texturable + Send + Sync>,
}

impl Lambertian {
    /// Creates a new diffuse material with the given albedo.
    pub fn new(texture: Box<dyn texturable::Texturable + Send + Sync>) -> Self {
        Self { texture }
    }
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

        let scattered_direction = hit_record.pdf.generate(rng);
        let scattered_ray = ray::Ray::new(
            &hit_record.hit.point,
            &scattered_direction,
            Some(hit_record.hit.ray.time),
        );
        let pdf_value = hit_record.pdf.value(scattered_ray.direction);
        if pdf_value <= 0.0 {
            tracker::add_sample_stat(tracker::Stat::new(
                tracker::LAMBERTIAN_SAMPLE,
                sample_start.elapsed(),
            ));
            return vec::Vec3::new(0.0, 0.0, 0.0);
        }
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
            let scattering_pdf = cosine::CosinePDF::new(&hit_record.hit.normal)
                .value(scattered_ray.direction);

            return (attenuation * bounce * scattering_pdf) / pdf_value;
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
