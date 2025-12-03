pub mod types;
pub mod traits;
pub mod materials;

use rand::Rng;

use crate::types::vec::Vec3;
use crate::types::ray::Ray;
use crate::types::camera::Camera;
use crate::types::scene::Scene;
use crate::traits::hittable::Hittable;
use crate::traits::hittable::HitRecord;

pub fn raytrace(
    rng: &mut rand::rngs::ThreadRng,
    width: u32,
    height: u32,
    camera: &Camera,
    scene: &Scene,
    ns: Option<u32>,
    max_depth: Option<u32>,
) -> Vec<u8> {
    let mut data = vec![0; (width * height * 3) as usize];
    let ns = ns.unwrap_or(50);
    let max_depth = max_depth.unwrap_or(8);

    for y in 0..height {
        for x in 0..width {
            let mut col = Vec3::new(0.0, 0.0, 0.0);
            for _s in 0..ns {
                let u = (x as f32 + rng.random::<f32>()) / width as f32;
                let v = (y as f32 + rng.random::<f32>()) / height as f32;

                let r = camera.get_ray(u, v);

                if let Some(hit) = scene.hit(&r, 0.001, f32::MAX) {
                    col = col + hit.sampleable.sample(rng, &hit, &scene, max_depth);
                }
            }
            col = col / ns as f32;
            col = Vec3::new(col.x.sqrt(), col.y.sqrt(), col.z.sqrt()); // gamma correction

            let offset = (y * width + x) * 3;
            data[offset as usize] = (col.x * 255.99) as u8; // R
            data[(offset + 1) as usize] = (col.y * 255.99) as u8; // G
            data[(offset + 2) as usize] = (col.z * 255.99) as u8; // B
        }
    }

    data
}