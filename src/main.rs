//! Binary entry point that renders the demo scene to `output.png`.
extern crate image;
extern crate rand;

use std::path::Path;

use rustray::core::scene;
use rustray::raytrace;

fn main() {
    let mut rng = rand::rng();

    let scene_path = Path::new("scenes/bouncing_spheres.toml");
    let (scene, camera, render_settings) = match scene::Scene::load_from_file(scene_path, &mut rng)
    {
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

    match image::save_buffer(
        &Path::new("output.png"),
        data.as_slice(),
        render_settings.width,
        ny,
        image::ColorType::Rgb8,
    ) {
        Ok(_) => println!("Image saved to output.png"),
        Err(e) => eprintln!("Failed to save image: {}", e),
    }
}
