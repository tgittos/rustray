use crate::core::vec;
use crate::textures::color;
use crate::traits::{hittable, texturable};

pub struct CheckerTexture {
    pub color1: color::ColorTexture,
    pub color2: color::ColorTexture,
    pub inv_scale: f32,
}

impl CheckerTexture {
    pub fn new(color1: color::ColorTexture, color2: color::ColorTexture, scale: f32) -> Self {
        CheckerTexture {
            color1,
            color2,
            inv_scale: 1.0 / scale,
        }
    }
}

impl texturable::Texturable for CheckerTexture {
    fn sample(&self, hit: &hittable::Hit) -> vec::Vec3 {
        // Use world-space position so large spheres (like the ground) don't collapse to bands near the poles.
        let x = (hit.point.x * self.inv_scale).floor() as i32;
        let y = (hit.point.y * self.inv_scale).floor() as i32;
        let z = (hit.point.z * self.inv_scale).floor() as i32;
        if (x + y + z) % 2 == 0 {
            self.color1.sample(hit)
        } else {
            self.color2.sample(hit)
        }
    }
}
