//! Binary entry point that renders the demo scene to `output.png`.
extern crate image;
extern crate rand;

use rand::Rng;
use std::path::Path;

use rustray::core::{camera, scene, vec};
use rustray::materials::{dielectric, diffuse, metallic};
use rustray::primitives::{skybox, sphere};
use rustray::raytrace;
use rustray::traits::{renderable, sampleable};

fn main() {
    let mut rng = rand::rng();

    let nx = 1200;
    //let nx = 400;
    let ar = 16.0 / 9.0;
    let ny = (nx as f32 / ar) as u32;
    let ns = 100; // samples per pixel
    // let ns = 500; // samples per pixel
    // let ns = 1000; // samples per pixel
    let max_depth: u32 = 50; // configurable bounce limit

    println!(
        "Rendering a {}x{} image with {} samples per pixel and max depth {}",
        nx, ny, ns, max_depth
    );

    // scene setup
    let camera_config = camera::CameraConfig {
        origin: vec::Vec3::new(13.0, 2.0, 3.0),
        look_at: vec::Vec3::new(0.0, 0.0, 0.0),
        up: vec::Vec3::new(0.0, 1.0, 0.0),
        aspect_ratio: ar,
        viewport_height: 2.0,
        focal_length: 10.0,
        aperture: 0.1,
        vertical_fov: 20.0,
    };
    let camera = camera::Camera::with_config(camera_config);
    let mut scene = scene::Scene::new();

    for i in -11..11 {
        for j in -11..11 {
            let choose_moving: bool = rng.random::<f32>() < 0.5;
            let choose_mat: f32 = rng.random::<f32>();
            let center = vec::Vec3::new(
                i as f32 + 0.9 * rng.random::<f32>(),
                0.2,
                j as f32 + 0.9 * rng.random::<f32>(),
            );

            if (center - vec::Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Box<dyn sampleable::Sampleable>;
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = vec::random(&mut rng) * vec::random(&mut rng);
                    sphere_material = Box::new(diffuse::Diffuse::new(&albedo));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = vec::random(&mut rng) * vec::random(&mut rng);
                    let fuzz = rng.random::<f32>() * 0.5;
                    sphere_material = Box::new(metallic::Metallic::new(&albedo, fuzz));
                } else {
                    // glass
                    sphere_material = Box::new(dielectric::Dielectric::new(1.5));
                }

                if choose_moving {
                    let center_end = center + vec::Vec3::new(0.0, 0.5 * rng.random::<f32>(), 0.0);
                    scene.add_object(Box::new(renderable::create_renderable(
                        Box::new(sphere::Sphere::moving(&center, &center_end, 0.0, 1.0, 0.2)),
                        sphere_material,
                    )));
                } else {
                    scene.add_object(Box::new(renderable::create_renderable(
                        Box::new(sphere::Sphere::new(&center, 0.2)),
                        sphere_material,
                    )));
                }
            }
        }
    }

    let left_sphere = renderable::create_renderable(
        Box::new(sphere::Sphere::new(&vec::Vec3::new(-4.0, 1.0, 0.0), 1.0)),
        Box::new(diffuse::Diffuse::new(&vec::Vec3::new(0.4, 0.2, 0.1))),
    );

    let center_sphere = renderable::create_renderable(
        Box::new(sphere::Sphere::new(&vec::Vec3::new(0.0, 1.0, 0.0), 1.0)),
        Box::new(dielectric::Dielectric::new(1.5)),
    );

    let right_sphere = renderable::create_renderable(
        Box::new(sphere::Sphere::new(&vec::Vec3::new(4.0, 1.0, 0.0), 1.0)),
        Box::new(metallic::Metallic::new(&vec::Vec3::new(0.7, 0.6, 0.5), 0.0)),
    );

    let world = renderable::create_renderable(
        Box::new(sphere::Sphere::new(
            &vec::Vec3::new(0.0, -1000.0, 0.0),
            1000.0,
        )),
        Box::new(diffuse::Diffuse::new(&vec::Vec3::new(0.5, 0.5, 0.5))),
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

    scene.build_bvh(&mut rng);

    let data = raytrace(&mut rng, nx, ar, &camera, &scene, Some(ns), Some(max_depth));

    match image::save_buffer(
        &Path::new("output.png"),
        data.as_slice(),
        nx,
        ny,
        image::ColorType::Rgb8,
    ) {
        Ok(_) => println!("Image saved to output.png"),
        Err(e) => eprintln!("Failed to save image: {}", e),
    }
}
