mod formats;
mod types;
mod traits;
mod materials;

use rand::prelude::*;

use formats::ppm;
use types::vec::Vec3;
use types::ray::Ray;
use traits::hittable::HitRecord;
use traits::hittable::Hittable;

fn hit<'a>(ray: &Ray, scene: &'a Vec<Box<dyn Hittable>>) -> Option<HitRecord<'a>> {
    let mut closest_so_far = f32::MAX;
    let mut hit_record: Option<HitRecord<'a>> = None;

    for object in scene {
        if let Some(record) = object.hit(ray, 0.0, closest_so_far) {
            closest_so_far = record.t;
            hit_record = Some(record);
        }
    }

    hit_record
}

fn main() {
    let mut rng = rand::rng();

    let nx = 200;
    let ny = 100;
    let ns = 100; // samples per pixel
    let max_depth: u32 = 8; // configurable bounce limit

    let white = Vec3::new(1.0, 1.0, 1.0);
    let blue = Vec3::new(0.5, 0.7, 1.0);

    println!("Writing sample PPM image...");
    let mut ppm_image = ppm::new_ppm_image(nx, ny, None);

    let camera = types::camera::Camera::new();

    // scene setup
    let mut scene = Vec::<Box<dyn Hittable>>::new();
    let sphere = types::sphere::Sphere::new(&Vec3::new(0.0, 0.0, -1.0), 0.5, None);
    let left_sphere = types::sphere::Sphere::new(&Vec3::new(-1.0, 0.0, -1.0), 0.5, Some(Box::new(materials::metallic::Metallic::new(&Vec3::new(0.8, 0.6, 0.2), 0.3))));
    let right_sphere = types::sphere::Sphere::new(&Vec3::new(1.0, 0.0, -1.0), 0.5, Some(Box::new(materials::metallic::Metallic::new(&Vec3::new(0.1, 0.2, 0.5), 1.0))));
    let world = types::sphere::Sphere::new(&Vec3::new(0.0, -100.5, -1.0), 100.0, None);
    let skybox = types::skybox::Skybox::new(&blue, &white);
    scene.push(Box::new(sphere));
    scene.push(Box::new(left_sphere));
    scene.push(Box::new(right_sphere));
    scene.push(Box::new(world));
    scene.push(Box::new(skybox));

    // fill with data
    for y in 0..ppm_image.height {
        for x in 0..ppm_image.width {
            let mut col = Vec3::new(0.0, 0.0, 0.0);
            for _s in 0..ns {
                let u = (x as f32 + rng.random::<f32>()) / ppm_image.width as f32;
                let v = (y as f32 + rng.random::<f32>()) / ppm_image.height as f32;

                let r = camera.get_ray(u, v);
                let hit_record = hit(&r, &scene);

                if let Some(hit_record) = hit_record {
                    col = col + hit_record.sampleable.sample(&mut rng, &hit_record, &scene, max_depth);
                }
            }
            col = col / ns as f32;
            col = Vec3::new(col.x.sqrt(), col.y.sqrt(), col.z.sqrt()); // gamma correction

            let offset = (y * ppm_image.width + x) * 3;
            ppm_image.data[offset] = (col.x * 255.99) as u8; // R
            ppm_image.data[offset + 1] = (col.y * 255.99) as u8; // G
            ppm_image.data[offset + 2] = (col.z * 255.99) as u8; // B
        }
    }

    match ppm::write_ppm("output.ppm", ppm_image) {
        Ok(_) => println!("PPM image written successfully."),
        Err(e) => eprintln!("Error writing PPM image: {}", e),
    }
}
