//! Binary entry point that renders the demo scene to `output.png`.
extern crate image;
extern crate rand;

use std::{
    env::{self},
    path::{Path, PathBuf},
};

use rustray::core::scene;
use rustray::{raytrace, raytrace_concurrent};

fn main() {
    let mut rng = rand::rng();

    let mut args = env::args();
    let program_name = args.next().unwrap_or_else(|| String::from("rustray"));
    let mut scene_path: Option<PathBuf> = None;
    let mut is_concurrent = false;
    let mut samples_override: Option<u32> = None;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--concurrent" => {
                is_concurrent = true;
            }
            "--spp" => {
                let value = args.next().unwrap_or_default();
                if value.is_empty() {
                    eprintln!(
                        "Missing value for --spp. Usage: {} [scene-file] [--concurrent] [--spp <samples>]",
                        program_name
                    );
                    std::process::exit(1);
                }
                match value.parse::<u32>() {
                    Ok(samples) => samples_override = Some(samples),
                    Err(err) => {
                        eprintln!("Invalid value for --spp ({}): {}", value, err);
                        std::process::exit(1);
                    }
                }
            }
            _ if arg.starts_with("--spp=") => {
                let value = arg.trim_start_matches("--spp=");
                match value.parse::<u32>() {
                    Ok(samples) => samples_override = Some(samples),
                    Err(err) => {
                        eprintln!("Invalid value for --spp ({}): {}", value, err);
                        std::process::exit(1);
                    }
                }
            }
            _ if arg.starts_with("--") => {
                eprintln!(
                    "Unknown option: {}. Usage: {} [scene-file] [--concurrent] [--spp <samples>]",
                    arg, program_name
                );
                std::process::exit(1);
            }
            _ => {
                if scene_path.is_some() {
                    eprintln!(
                        "Unexpected extra argument: {}. Usage: {} [scene-file] [--concurrent] [--spp <samples>]",
                        arg, program_name
                    );
                    std::process::exit(1);
                }
                scene_path = Some(PathBuf::from(arg));
            }
        }
    }

    let scene_path = scene_path.unwrap_or_else(|| PathBuf::from("scenes/bouncing_spheres.toml"));

    if !scene_path.is_file() {
        eprintln!(
            "Scene file not found: {}. Usage: {} [scene-file] [--concurrent] [--spp <samples>]",
            scene_path.display(),
            program_name
        );
        std::process::exit(1);
    }

    let mut render = match scene::load_from_file(&mut rng, scene_path.as_path()) {
        Ok(result) => result,
        Err(err) => {
            eprintln!(
                "Failed to load scene from {}: {}",
                scene_path.display(),
                err
            );
            std::process::exit(1);
        }
    };

    if let Some(samples) = samples_override {
        render.samples = samples;
    }

    let data = if is_concurrent {
        let cpus = num_cpus::get();
        println!(
            "Rendering a {}x{} image with {} samples per pixel and max depth {} using {} threads",
            render.width,
            render.width as f32 / render.camera.aspect_ratio,
            render.samples,
            render.depth,
            cpus
        );
        raytrace_concurrent(&render)
    } else {
        println!(
            "Rendering a {}x{} image with {} samples per pixel and max depth {}",
            render.width,
            render.width as f32 / render.camera.aspect_ratio,
            render.samples,
            render.depth
        );
        raytrace(&mut rng, &render)
    };

    let filename = scene_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("output");

    match image::save_buffer(
        &Path::new(&format!("samples/{}.png", filename)),
        data.as_slice(),
        render.width,
        (render.width as f32 / render.camera.aspect_ratio) as u32,
        image::ColorType::Rgb8,
    ) {
        Ok(_) => println!("Image saved to samples/{}.png", filename),
        Err(e) => eprintln!("Failed to save image: {}", e),
    }
}
