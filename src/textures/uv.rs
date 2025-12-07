extern crate image;

use crate::traits::texturable;
use crate::core::{vec, interval};
use crate::traits::hittable;

pub struct UvTexture {
    data: Vec<u8>,
    width: u32,
    height: u32,
}

impl UvTexture {
    pub fn new(path: &str) -> Self {
        let img = image::open(path)
            .expect("Failed to open UV texture image");
        let img = img.to_rgb8();
        let (width, height) = img.dimensions();
        let data = img.into_raw();
        UvTexture {
            data,
            width,
            height,
        }
    }
}

impl texturable::Texturable for UvTexture {
    fn sample(&self, hit: &hittable::Hit) -> vec::Vec3 {
        let u = interval::Interval::new(0.0, 1.0).clamp(hit.u);
        let v = interval::Interval::new(0.0, 1.0).clamp(hit.v);
        let i = ((u * self.width as f32) as u32).min(self.width - 1);
        let j = (((1.0 - v) * self.height as f32) as u32).min(self.height - 1);
        let pixel_index = ((j * self.width + i) * 3) as usize;
        let r = self.data[pixel_index] as f32 / 255.0;
        let g = self.data[pixel_index + 1] as f32 / 255.0;
        let b = self.data[pixel_index + 2] as f32 / 255.0;
        vec::Vec3::new(r, g, b)
    }
}