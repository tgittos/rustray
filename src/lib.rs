//! A ray tracing library in Rust.
//!
//! Provides core components for ray tracing, including vectors, rays, cameras, scenes,
//! primitives, materials, and rendering functionality.
pub mod core;
pub mod geometry;
pub mod materials;
pub mod math;
pub mod stats;
pub mod textures;
pub mod traits;

use rand::Rng;
use rayon::prelude::*;
use std::time;

use crate::core::render;
use crate::math::vec;
use crate::stats::tracker;
use crate::traits::renderable::Renderable;

#[derive(Clone, Copy)]
pub(crate) struct ChunkBounds {
    pub x_start: u32,
    pub x_end: u32,
    pub y_start: u32,
    pub y_end: u32,
}

impl ChunkBounds {
    pub fn width(&self) -> u32 {
        self.x_end - self.x_start
    }

    pub fn height(&self) -> u32 {
        self.y_end - self.y_start
    }
}

pub(crate) struct ChunkOutput {
    pub bounds: ChunkBounds,
    pub data: Vec<u8>,
}

pub(crate) fn image_height(render: &render::Render) -> u32 {
    (render.width as f32 / render.camera.aspect_ratio) as u32
}

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
pub fn raytrace(rng: &mut rand::rngs::ThreadRng, render: &render::Render) -> Vec<u8> {
    let height = image_height(render);
    tracker::reset();
    let render_start = time::Instant::now();

    let full_frame = ChunkBounds {
        x_start: 0,
        x_end: render.width,
        y_start: 0,
        y_end: height,
    };
    let chunk = raytrace_chunk(rng, render, full_frame);
    let image_data = assemble_chunks(&[chunk], render.width, height);

    let wall_time = render_start.elapsed();
    let stats = tracker::get_stats();

    println!("Rendering Stats:");
    println!("--------------------------");
    println!("Total Hits: {}", stats.total_hits());
    println!("Total Samples: {}", stats.total_samples());
    vec![
        tracker::SCENE_HIT,
        tracker::LAMBERTIAN_HIT,
        tracker::LAMBERTIAN_SAMPLE,
        tracker::METALLIC_HIT,
        tracker::METALLIC_SAMPLE,
        tracker::DIELECTRIC_HIT,
        tracker::DIELECTRIC_SAMPLE,
        tracker::DIFFUSE_LIGHT_SAMPLE,
    ]
    .iter()
    .for_each(|stat_name| {
        println!(
            "Stat: {}\n  P50: {:?}\n  P90: {:?}\n  P99: {:?}\n",
            stat_name,
            stats.p50_by_name(stat_name),
            stats.p90_by_name(stat_name),
            stats.p99_by_name(stat_name)
        );
    });
    println!(
        "Total Hit Time: {}",
        format_duration(stats.total_hit_time())
    );
    println!(
        "Total Sample Time: {}",
        format_duration(stats.total_sample_time())
    );
    println!("--------------------------");
    println!("Render Wall Time: {}", format_duration(wall_time));
    println!("--------------------------");

    image_data
}

pub fn raytrace_concurrent(render: &render::Render) -> Vec<u8> {
    let height = image_height(render);
    tracker::reset();
    let render_start = time::Instant::now();

    let num_threads = num_cpus::get();
    let chunk_height = (height + num_threads as u32 - 1) / num_threads as u32;

    let chunks: Vec<ChunkBounds> = (0..num_threads)
        .map(|i| {
            let y_start = i as u32 * chunk_height;
            let y_end = ((i as u32 + 1) * chunk_height).min(height);
            ChunkBounds {
                x_start: 0,
                x_end: render.width,
                y_start,
                y_end,
            }
        })
        .collect();

    let chunk_outputs: Vec<ChunkOutput> = chunks
        .into_par_iter()
        .map(|chunk_bounds| {
            let mut local_rng = rand::rng();
            raytrace_chunk(&mut local_rng, render, chunk_bounds)
        })
        .collect();

    let image_data = assemble_chunks(&chunk_outputs, render.width, height);

    let wall_time = render_start.elapsed();
    let stats = tracker::get_stats();

    println!("Rendering Stats:");
    println!("--------------------------");
    println!("Total Hits: {}", stats.total_hits());
    println!("Total Samples: {}", stats.total_samples());
    vec![
        tracker::SCENE_HIT,
        tracker::LAMBERTIAN_HIT,
        tracker::LAMBERTIAN_SAMPLE,
        tracker::METALLIC_HIT,
        tracker::METALLIC_SAMPLE,
        tracker::DIELECTRIC_HIT,
        tracker::DIELECTRIC_SAMPLE,
        tracker::DIFFUSE_LIGHT_SAMPLE,
    ]
    .iter()
    .for_each(|stat_name| {
        println!(
            "Stat: {}\n  P50: {:?}\n  P90: {:?}\n  P99: {:?}\n",
            stat_name,
            stats.p50_by_name(stat_name),
            stats.p90_by_name(stat_name),
            stats.p99_by_name(stat_name)
        );
    });
    println!(
        "Total Hit Time: {}",
        format_duration(stats.total_hit_time())
    );
    println!(
        "Total Sample Time: {}",
        format_duration(stats.total_sample_time())
    );
    println!("--------------------------");
    println!("Render Wall Time: {}", format_duration(wall_time));
    println!("--------------------------");
    image_data
}

fn format_duration(dur: time::Duration) -> String {
    let hours = dur.as_secs() / 3600;
    let minutes = (dur.as_secs() % 3600) / 60;
    let seconds = dur.as_secs() % 60;
    let millis = dur.subsec_millis();
    format!("{}h {}m {}s {}ms", hours, minutes, seconds, millis)
}

pub(crate) fn raytrace_chunk(
    rng: &mut rand::rngs::ThreadRng,
    render: &render::Render,
    bounds: ChunkBounds,
) -> ChunkOutput {
    let height = image_height(render) as f32;
    let sqrt_n = (render.samples.max(1) as f32).sqrt() as u32;
    let ns = sqrt_n * sqrt_n; // Ensure ns is a perfect square
    let pss = 1.0 / (ns as f32);
    let recip_sqrt_n = 1.0 / (sqrt_n as f32);
    let max_depth = render.depth;
    let row_width = bounds.width() as usize * 3;
    let mut data = Vec::with_capacity(row_width * bounds.height() as usize);

    for y in bounds.y_start..bounds.y_end {
        for x in bounds.x_start..bounds.x_end {
            let mut col = vec::Vec3::new(0.0, 0.0, 0.0);

            for i in 0..sqrt_n {
                for j in 0..sqrt_n {
                    let u = (x as f32 + (i as f32 + rng.random::<f32>()) * recip_sqrt_n)
                        / render.width as f32;
                    let v = (y as f32 + (j as f32 + rng.random::<f32>()) * recip_sqrt_n) / height;

                    let r = render.camera.get_ray(rng, u, v);

                    if let Some(hit) = render.scene.hit(&r, 0.001, f32::MAX) {
                        col = col + hit.renderable.sample(rng, &hit, &render.scene, max_depth);
                    }
                }
            }

            col = col * pss;
            col = col.sqrt(); // Gamma correction

            data.push((col.x * 255.99) as u8);
            data.push((col.y * 255.99) as u8);
            data.push((col.z * 255.99) as u8);
        }
    }

    ChunkOutput { bounds, data }
}

pub(crate) fn assemble_chunks(chunks: &[ChunkOutput], width: u32, height: u32) -> Vec<u8> {
    let frame_row_stride = width as usize * 3;
    let mut image = vec![0_u8; frame_row_stride * height as usize];

    for chunk in chunks {
        let chunk_row_stride = chunk.bounds.width() as usize * 3;
        for (row_idx, y) in (chunk.bounds.y_start..chunk.bounds.y_end).enumerate() {
            let dest_row = (height - 1 - y) as usize;
            let dest_offset = dest_row * frame_row_stride + chunk.bounds.x_start as usize * 3;
            let src_offset = row_idx * chunk_row_stride;
            let src_end = src_offset + chunk_row_stride;

            image[dest_offset..dest_offset + chunk_row_stride]
                .copy_from_slice(&chunk.data[src_offset..src_end]);
        }
    }

    image
}
