//! A ray tracing library in Rust.
//!
//! Provides core components for ray tracing, including vectors, rays, cameras, scenes,
//! primitives, materials, and rendering functionality.
pub mod core;
pub mod materials;
pub mod primitives;
pub mod traits;
pub mod utils;

use rand::Rng;
use std::time;

use crate::core::{camera, scene, vec};
use crate::traits::renderable::Renderable;
use crate::utils::stats;

/// Renders the given scene to an RGB buffer using stochastic sampling.
///
/// # Arguments
/// * `rng` - Random number generator used for jittered sampling.
/// * `width`/`height` - Output dimensions in pixels.
/// * `camera` - Camera used to generate view rays.
/// * `scene` - Collection of renderable objects to trace against.
/// * `ns` - Optional number of samples per pixel (defaults to 50).
/// * `max_depth` - Optional recursion limit for ray bounces (defaults to 8).
///
/// # Returns
/// A flat RGB buffer in row-major order with gamma correction applied.
pub fn raytrace(
    rng: &mut rand::rngs::ThreadRng,
    width: u32,
    height: u32,
    camera: &camera::Camera,
    scene: &scene::Scene,
    ns: Option<u32>,
    max_depth: Option<u32>,
) -> Vec<u8> {
    let mut stats = stats::Stats::new();
    let mut data = vec![0; (width * height * 3) as usize];
    let ns = ns.unwrap_or(50);
    let max_depth = max_depth.unwrap_or(8);

    for y in 0..height {
        for x in 0..width {
            let now = time::Instant::now();
            let mut hit_elapsed = time::Duration::new(0, 0);
            let mut sample_elapsed = time::Duration::new(0, 0);
            let mut col = vec::Vec3::new(0.0, 0.0, 0.0);

            for _s in 0..ns {
                let u = (x as f32 + rng.random::<f32>()) / width as f32;
                let v = (y as f32 + rng.random::<f32>()) / height as f32;

                let r = camera.get_ray(u, v);

                if let Some(hit) = scene.hit(&r, 0.001, f32::MAX) {
                    hit_elapsed = now.elapsed();
                    col = col + hit.renderable.sample(rng, &hit, &scene, max_depth);
                    sample_elapsed = now.elapsed() - hit_elapsed;
                }
            }

            stats.add_stat(stats::Stat::new(x, y, hit_elapsed, sample_elapsed));

            col = col / ns as f32;
            col = vec::Vec3::new(col.x.sqrt(), col.y.sqrt(), col.z.sqrt()); // gamma correction

            let offset = (y * width + x) * 3;
            data[offset as usize] = (col.x * 255.99) as u8; // R
            data[(offset + 1) as usize] = (col.y * 255.99) as u8; // G
            data[(offset + 2) as usize] = (col.z * 255.99) as u8; // B
        }
    }

    let (p50_hit, p50_sample) = stats.p50();
    let (p90_hit, p90_sample) = stats.p90();
    let (p99_hit, p99_sample) = stats.p99();
    let total_hit = stats.total_hit_time();
    let total_sample = stats.total_sample_time();
    let total = stats.total_time();

    println!("P50 Hit Time: {:?}", p50_hit);
    println!("P50 Sample Time: {:?}", p50_sample);
    println!("P90 Hit Time: {:?}", p90_hit);
    println!("P90 Sample Time: {:?}", p90_sample);
    println!("P99 Hit Time: {:?}", p99_hit);
    println!("P99 Sample Time: {:?}", p99_sample);
    println!("Total Hit Time: {:?}", total_hit);
    println!("Total Sample Time: {:?}", total_sample);
    println!("Total Time: {:?}", total);

    data
}
