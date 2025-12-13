//! Lambertian diffuse material that scatters light uniformly.
use std::time;

use crate::core::{ray, scene};
use crate::math::{onb, pdf, vec};
use crate::stats::tracker;
use crate::traits::renderable::Renderable;
use crate::traits::{hittable, sampleable, texturable};

/// Diffuse surface with a constant albedo.
pub struct Lambertian {
    pub texture: Box<dyn texturable::Texturable + Send + Sync>,
    pdf: Box<dyn pdf::PDF + Send + Sync>,
}

struct CosinePDF;
impl pdf::PDF for CosinePDF {
    fn sample(
        &self,
        _in_ray: &ray::Ray,
        hit_record: &hittable::HitRecord,
        out_ray: &ray::Ray,
    ) -> f32 {
        let unit_direction = vec::unit_vector(&out_ray.direction);
        let cosine = hit_record.hit.normal.dot(&unit_direction);
        if cosine < 0.0 {
            0.0
        } else {
            cosine / std::f32::consts::PI
        }
    }
}

impl Lambertian {
    /// Creates a new diffuse material with the given albedo.
    pub fn new(texture: Box<dyn texturable::Texturable + Send + Sync>) -> Self {
        Self {
            texture,
            pdf: Box::new(CosinePDF {}),
        }
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
        let uvw = onb::ONB::build_from_w(&hit_record.hit.normal);
        let scatter_direction = uvw.local(&random_cosine_direction(rng));
        let scattered_ray = ray::Ray::new(
            &hit_record.hit.point,
            &vec::unit_vector(&scatter_direction),
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
            let pdf = self
                .pdf
                .sample(&hit_record.hit.ray, hit_record, &scattered_ray);

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
