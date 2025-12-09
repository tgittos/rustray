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

    let render = match scene::load_from_file(&mut rng, scene_path.as_path()) {
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

    println!(
        "Rendering a {}x{} image with {} samples per pixel and max depth {}",
        render.width,
        render.width as f32 * render.camera.aspect_ratio,
        render.samples,
        render.depth
    );

    let data = raytrace(&mut rng, &render);

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
