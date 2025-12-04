mod formats;

use rustray::core::{camera, scene, vec};
use rustray::materials::{dielectric, diffuse, metallic};
use rustray::primitives::{skybox, sphere};
use rustray::raytrace;
use rustray::traits::renderable;

use formats::ppm;

fn main() {
    let mut rng = rand::rng();

    // let nx = 800;
    // let ny = 600;
    let nx = 200;
    let ny = 100;
    let ns = 50; // samples per pixel
    let max_depth: u32 = 8; // configurable bounce limit

    println!(
        "Rendering a {}x{} image with {} samples per pixel and max depth {}",
        nx, ny, ns, max_depth
    );

    // scene setup
    let camera_config = camera::CameraConfig {
        origin: vec::Vec3::new(0.0, 0.0, 0.0),
        look_at: vec::Vec3::new(0.0, 0.0, -1.0),
        up: vec::Vec3::new(0.0, 1.0, 0.0),
        aspect_ratio: nx as f32 / ny as f32,
        viewport_height: 2.0,
        focal_length: 1.0,
        aperture: 0.1,
        vertical_fov: 75.0,
    };
    let camera = camera::Camera::with_config(camera_config);
    let mut scene = scene::Scene::new();

    let center_sphere = renderable::create_renderable(
        Box::new(sphere::Sphere::new(&vec::Vec3::new(0.0, 0.0, -1.0), 0.5)),
        Box::new(diffuse::Diffuse::new(&vec::Vec3::new(0.7, 0.3, 0.3))),
    );

    let left_sphere = renderable::create_renderable(
        Box::new(sphere::Sphere::new(&vec::Vec3::new(-1.0, 0.0, -1.0), -0.5)),
        Box::new(dielectric::Dielectric::new(1.5)),
    );

    let right_sphere = renderable::create_renderable(
        Box::new(sphere::Sphere::new(&vec::Vec3::new(1.0, 0.0, -1.0), 0.5)),
        Box::new(metallic::Metallic::new(&vec::Vec3::new(0.8, 0.6, 0.2), 0.0)),
    );

    let world = renderable::create_renderable(
        Box::new(sphere::Sphere::new(
            &vec::Vec3::new(0.0, -100.5, -1.0),
            100.0,
        )),
        Box::new(diffuse::Diffuse::new(&vec::Vec3::new(0.8, 0.8, 0.0))),
    );

    let skybox_primitive = skybox::Skybox::new(
        &vec::Vec3::new(0.5, 0.7, 1.0),
        &vec::Vec3::new(1.0, 1.0, 1.0),
    );
    let skybox =
        renderable::create_renderable(Box::new(skybox_primitive), Box::new(skybox_primitive));

    scene.add_object(Box::new(center_sphere));
    scene.add_object(Box::new(left_sphere));
    scene.add_object(Box::new(right_sphere));
    scene.add_object(Box::new(world));
    scene.add_object(Box::new(skybox));

    let data = raytrace(&mut rng, nx, ny, &camera, &scene, Some(ns), Some(max_depth));

    let mut ppm_image = ppm::new_ppm_image(nx as usize, ny as usize, None);

    for y in 0..ppm_image.height {
        for x in 0..ppm_image.width {
            let offset = (y * ppm_image.width + x) * 3;
            ppm_image.data[offset] = (data[offset]) as u8; // R
            ppm_image.data[offset + 1] = (data[offset + 1]) as u8; // G
            ppm_image.data[offset + 2] = (data[offset + 2]) as u8; // B
        }
    }

    match ppm::write_ppm("output.ppm", ppm_image) {
        Ok(_) => println!("PPM image written successfully."),
        Err(e) => eprintln!("Error writing PPM image: {}", e),
    }
}
