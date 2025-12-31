//! A ray tracing library in Rust.
//!
//! Provides core components for ray tracing, including vectors, rays, cameras, scenes,
//! primitives, materials, and rendering functionality.
pub mod core;
pub mod geometry;
pub mod materials;
pub mod math;
pub mod samplers;
pub mod stats;
pub mod textures;
pub mod traits;

use rayon::prelude::*;
use std::time;

use crate::core::ray;
use crate::core::render;
use crate::core::scene;
use crate::math::pdf;
use crate::math::vec;
use crate::samplers::monte_carlo::MonteCarloSampler;
use crate::samplers::sampleable::Sampleable;
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

    println!("Wall time: {}", format_duration(wall_time));

    image_data
}

pub fn raytrace_concurrent(render: &render::Render) -> Vec<u8> {
    let height = image_height(render);
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

    println!("Wall time: {}", format_duration(wall_time));

    image_data
}

pub(crate) fn raytrace_chunk(
    rng: &mut rand::rngs::ThreadRng,
    render: &render::Render,
    bounds: ChunkBounds,
) -> ChunkOutput {
    let height = image_height(render);
    let sampler = MonteCarloSampler::new(
        render.samples,
        render.depth,
        &render.camera,
        &render.scene,
        trace_ray,
    );
    let row_width = bounds.width() as usize * 3;
    let mut data = Vec::with_capacity(row_width * bounds.height() as usize);

    for y in bounds.y_start..bounds.y_end {
        for x in bounds.x_start..bounds.x_end {
            let mut col = sampler.sample_pixel(rng, x, y, render.width, height);
            col = col.sqrt(); // Gamma correction

            data.push((col.x * 255.99) as u8);
            data.push((col.y * 255.99) as u8);
            data.push((col.z * 255.99) as u8);
        }
    }

    ChunkOutput { bounds, data }
}

fn trace_ray(
    rng: &mut rand::rngs::ThreadRng,
    scene: &scene::Scene,
    ray: &ray::Ray,
    max_depth: u32,
) -> vec::Vec3 {
    let mut current_ray = *ray;
    let mut throughput = vec::Vec3::new(1.0, 1.0, 1.0);
    let mut radiance = vec::Vec3::new(0.0, 0.0, 0.0);
    let mut remaining_depth = max_depth;

    loop {
        let Some(hit_record) = scene.hit(&current_ray, 0.001, f32::MAX) else {
            // no hit, no color contribution
            break;
        };

        let emitted = hit_record.renderable.emit(&hit_record);
        let scatter_record = if remaining_depth > 0 {
            hit_record
                .renderable
                .scatter(rng, &hit_record, remaining_depth)
        } else {
            None
        };

        radiance = radiance + throughput * emitted;

        let Some(scatter_record) = scatter_record else {
            break;
        };

        remaining_depth = remaining_depth.saturating_sub(1);

        if let Some(specular_ray) = scatter_record.scattered_ray {
            throughput = throughput * scatter_record.attenuation;
            current_ray = specular_ray;
            continue;
        }

        let Some(scatter_pdf) = scatter_record.scatter_pdf.as_ref() else {
            break;
        };

        let mut mixed_pdf: Option<pdf::MixturePDF<'_>> = None;
        let sample_pdf: &dyn pdf::PDF = if scatter_record.use_light_pdf {
            if let Some(pdf) = scene.light_pdf(&hit_record, scatter_pdf.as_ref()) {
                mixed_pdf = Some(pdf);
                mixed_pdf.as_ref().unwrap()
            } else {
                scatter_pdf.as_ref()
            }
        } else {
            scatter_pdf.as_ref()
        };

        let scatter_direction = sample_pdf.generate(rng);
        let scattered_ray = ray::Ray::new(
            &hit_record.hit.point,
            &scatter_direction,
            Some(hit_record.hit.ray.time),
        );

        let pdf_value = sample_pdf.value(scattered_ray.direction);
        if pdf_value <= 0.0 {
            break;
        }

        if scatter_record.use_light_pdf && mixed_pdf.is_some() {
            let scattering_pdf = scatter_pdf.value(scattered_ray.direction);
            throughput = throughput * scatter_record.attenuation * scattering_pdf / pdf_value;
        } else {
            throughput = throughput * scatter_record.attenuation;
        }
        current_ray = scattered_ray;
    }

    radiance
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

fn format_duration(dur: time::Duration) -> String {
    let hours = dur.as_secs() / 3600;
    let minutes = (dur.as_secs() % 3600) / 60;
    let seconds = dur.as_secs() % 60;
    let millis = dur.subsec_millis();
    format!("{}h {}m {}s {}ms", hours, minutes, seconds, millis)
}
