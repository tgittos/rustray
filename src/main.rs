mod formats;
mod types;

use formats::ppm;
use types::vec::Vec3;
use types::ray::Ray;

fn sample(ray: &types::ray::Ray) -> Vec3 {
    let unit_direction = types::vec::unit_vector(&ray.direction);
    let t = 0.5 * (unit_direction.y + 1.0);
    let white = Vec3::new(1.0, 1.0, 1.0);
    let blue = Vec3::new(0.5, 0.7, 1.0);
    // lerp blue and white
    white * (1.0 - t) + blue * t
}

fn main() {
    println!("Writing sample PPM image...");
    let mut ppm_image = ppm::new_ppm_image(200, 100, None);

    // mock camera params
    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    // fill with data
    for y in 0..ppm_image.height {
        for x in 0..ppm_image.width {
            let u = x as f32 / ppm_image.width as f32;
            let v = y as f32 / ppm_image.height as f32;

            let direction = lower_left_corner + horizontal * u + vertical * v;
            let r = types::ray::Ray::new(&origin, &direction);
            let color = sample(&r);

            let offset = (y * ppm_image.width + x) * 3;
            ppm_image.data[offset] = (color.x * 255.99) as u8; // R
            ppm_image.data[offset + 1] = (color.y * 255.99) as u8; // G
            ppm_image.data[offset + 2] = (color.z * 255.99) as u8; // B
        }
    }

    match ppm::write_ppm("output.ppm", ppm_image) {
        Ok(_) => println!("PPM image written successfully."),
        Err(e) => eprintln!("Error writing PPM image: {}", e),
    }
}


