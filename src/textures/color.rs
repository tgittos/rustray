use serde::{Deserialize, Serialize};

use crate::core::vec;
use crate::traits::texturable;

#[derive(Serialize, Deserialize)]
pub struct ColorTexture {
    pub albedo: vec::Vec3,
}

impl ColorTexture {
    pub fn new(albedo: vec::Vec3) -> Self {
        ColorTexture { albedo }
    }

    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        ColorTexture {
            albedo: vec::Vec3::new(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0),
        }
    }
}

#[typetag::serde]
impl texturable::Texturable for ColorTexture {
    fn sample(&self, _hit_record: &crate::traits::hittable::Hit) -> vec::Vec3 {
        self.albedo
    }
}
