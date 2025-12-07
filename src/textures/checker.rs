use crate::core::vec;
use crate::traits::texturable;

pub struct CheckerTexture {
    pub color1: vec::Vec3,
    pub color2: vec::Vec3,
    pub scale: f32,
}

impl CheckerTexture {
    pub fn new(color1: vec::Vec3, color2: vec::Vec3, scale: f32) -> Self {
        CheckerTexture {
            color1,
            color2,
            scale,
        }
    }
}

impl texturable::Texturable for CheckerTexture {
    fn value(&self, _u: f32, _v: f32, p: &vec::Point3) -> vec::Vec3 {
        let sines = (self.scale * p.x).sin() * (self.scale * p.y).sin() * (self.scale * p.z).sin();
        if sines < 0.0 {
            self.color1
        } else {
            self.color2
        }
    }
}
