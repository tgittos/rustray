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
//use std::thread::available_parallelism;

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
    aspect_ratio: f32,
    camera: &camera::Camera,
    scene: &scene::Scene,
    ns: Option<u32>,
    max_depth: Option<u32>,
) -> Vec<u8> {
    let mut stats = stats::Stats::new();

    let height = (width as f32 / aspect_ratio) as u32;
    let ns = ns.unwrap_or(50);
    let max_depth = max_depth.unwrap_or(8);

    let pixel_cols = (0..height)
        .into_iter()
        .map(|y| {
            (0..width)
                .into_iter()
                .map(|x| {
                    let mut col = vec::Vec3::new(0.0, 0.0, 0.0);

                    for _s in 0..ns {
                        let u = (x as f32 + rng.random::<f32>()) / width as f32;
                        let v = (y as f32 + rng.random::<f32>()) / height as f32;

                        let r = camera.get_ray(rng, u, v);

                        let hit_start = time::Instant::now();
                        if let Some(hit) = scene.hit(&r, 0.001, f32::MAX) {
                            let hit_elapsed = hit_start.elapsed();

                            let sample_start = time::Instant::now();
                            col = col + hit.renderable.sample(rng, &hit, &scene, max_depth);
                            let sample_elapsed = sample_start.elapsed();

                            stats.add_stat(stats::Stat::new(hit_elapsed, sample_elapsed));
                        }
                    }

                    col = col / ns as f32;
                    col = col.sqrt(); // Gamma correction

                    col
                })
                .collect::<Vec<vec::Vec3>>()
        })
        .collect::<Vec<Vec<vec::Vec3>>>();

    let image_data = pixel_cols
        .into_iter()
        .flat_map(|row| {
            row.into_iter()
                .flat_map(|col| {
                    let r = (col.x * 255.99) as u8;
                    let g = (col.y * 255.99) as u8;
                    let b = (col.z * 255.99) as u8;
                    vec![r, g, b]
                })
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<u8>>();

    println!("Rendering Stats:");
    println!("--------------------------");
    println!("P50: {:?}", stats.p50());
    println!("P90: {:?}", stats.p90());
    println!("P99: {:?}", stats.p99());
    println!("Total Hit Time: {:?}", stats.total_hit_time());
    println!("Total Sample Time: {:?}", stats.total_sample_time());
    println!("--------------------------");
    println!("Total Time: {:?}", stats.total_time());
    println!("--------------------------");

    image_data
}
