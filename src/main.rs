mod formats;

use rustray::raytrace;
use rustray::types;
use rustray::materials;

use formats::ppm;

fn main() {
    let mut rng = rand::rng();

    // let nx = 800;
    // let ny = 600;
    let nx = 200;
    let ny = 100;
    let ns = 50; // samples per pixel
    let max_depth: u32 = 8; // configurable bounce limit

    println!("Rendering a {}x{} image with {} samples per pixel and max depth {}", nx, ny, ns, max_depth);

    // scene setup
    let camera_config = types::camera::CameraConfig {
        origin: types::vec::Vec3::new(0.0, 0.0, 0.0),
        look_at: types::vec::Vec3::new(0.0, 0.0, -1.0),
        up: types::vec::Vec3::new(0.0, 1.0, 0.0),
        aspect_ratio: nx as f32 / ny as f32,
        viewport_height: 2.0,
        focal_length: 1.0,
        aperture: 0.1,
        vertical_fov: 75.0,
    };
    let camera = types::camera::Camera::with_config(camera_config);
    let mut scene = types::scene::Scene::new();
    let sphere = types::sphere::Sphere::new(&types::vec::Vec3::new(0.0, 0.0, -1.0), 0.5, None);
    let left_sphere = types::sphere::Sphere::new(&types::vec::Vec3::new(-1.0, 0.0, -1.0), -0.5, Some(Box::new(materials::dielectric::Dielectric::new(1.5))));
    let right_sphere = types::sphere::Sphere::new(&types::vec::Vec3::new(1.0, 0.0, -1.0), 0.5, Some(Box::new(materials::metallic::Metallic::new(&types::vec::Vec3::new(0.1, 0.2, 0.5), 0.0))));
    let world = types::sphere::Sphere::new(&types::vec::Vec3::new(0.0, -100.5, -1.0), 100.0, Some(Box::new(materials::diffuse::Diffuse::new(&types::vec::Vec3::new(0.8, 0.8, 0.0)))));
    let skybox = types::skybox::Skybox::new(&types::vec::Vec3::new(0.5, 0.7, 1.0), &types::vec::Vec3::new(1.0, 1.0, 1.0));
    scene.add_object(Box::new(sphere));
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
