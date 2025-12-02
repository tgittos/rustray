mod formats;
mod types;
mod traits;

use rand::prelude::*;

use formats::ppm;
use types::vec::Vec3;
use types::ray::Ray;
use traits::hittable::Hittable;


fn sample(ray: &types::ray::Ray, scene: &Vec<Box<dyn Hittable>>) -> Vec3 {
    let unit_direction = types::vec::unit_vector(&ray.direction);
    let t = 0.5 * (unit_direction.y + 1.0);
    let white = Vec3::new(1.0, 1.0, 1.0);
    let blue = Vec3::new(0.5, 0.7, 1.0);

    for object in scene {
        if let Some(hit_record) = object.hit(ray, 0.0, f32::MAX) {
            return Vec3::new(hit_record.normal.x + 1.0, hit_record.normal.y + 1.0, hit_record.normal.z + 1.0) / 2.0;
        }
    }

    // lerp blue and white
    white * (1.0 - t) + blue * t
}

fn main() {
    let mut rng = rand::thread_rng();

    let nx = 200;
    let ny = 100;
    let ns = 100; // samples per pixel

    println!("Writing sample PPM image...");
    let mut ppm_image = ppm::new_ppm_image(nx, ny, None);

    // scene setup
    let camera = types::camera::Camera::new();
    let mut scene = (Vec::<Box<dyn Hittable>>::new());
    let sphere = types::sphere::Sphere::new(&Vec3::new(0.0, 0.0, -1.0), 0.5);
    let world = types::sphere::Sphere::new(&Vec3::new(0.0, -100.5, -1.0), 100.0);
    scene.push(Box::new(sphere));
    scene.push(Box::new(world));

    // fill with data
    for y in 0..ppm_image.height {
        for x in 0..ppm_image.width {
            let mut col = Vec3::new(0.0, 0.0, 0.0);
            for _s in 0..ns {
                let u = (x as f32 + rng.random::<f32>()) / ppm_image.width as f32;
                let v = (y as f32 + rng.random::<f32>()) / ppm_image.height as f32;

                let r = camera.get_ray(u, v);
                col = col + sample(&r, &scene);
            }
            col = col / ns as f32;

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


