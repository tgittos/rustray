//! Binary entry point that renders the demo scene to `output.png`.
extern crate image;
extern crate rand;

use std::{
    env::{self},
    path::{Path, PathBuf},
};

use rustray::core::scene;
use rustray::raytrace;

fn main() {
    let mut rng = rand::rng();

    let mut args = env::args();
    let program_name = args.next().unwrap_or_else(|| String::from("rustray"));
    let scene_path = args
        .next()
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("scenes/bouncing_spheres.toml"));

    if !scene_path.is_file() {
        eprintln!(
            "Scene file not found: {}. Usage: {} <scene-file>",
            scene_path.display(),
            program_name
        );
        std::process::exit(1);
    }

    let (scene, camera, render_settings) =
        match scene::Scene::load_from_file(scene_path.as_path(), &mut rng) {
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
    let ar = render_settings.aspect_ratio();
    let ny = (render_settings.width as f32 / ar) as u32;

    println!(
        "Rendering a {}x{} image with {} samples per pixel and max depth {}",
        render_settings.width, ny, render_settings.samples, render_settings.max_depth
    );

    let data = raytrace(
        &mut rng,
        render_settings.width,
        render_settings.aspect_ratio(),
        &camera,
        &scene,
        Some(render_settings.samples),
        Some(render_settings.max_depth),
    );

    let filename = scene_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("output");

    match image::save_buffer(
        &Path::new(&format!("samples/{}.png", filename)),
        data.as_slice(),
        render_settings.width,
        ny,
        image::ColorType::Rgb8,
    ) {
        Ok(_) => println!("Image saved to samples/{}.png", filename),
        Err(e) => eprintln!("Failed to save image: {}", e),
    }
}
