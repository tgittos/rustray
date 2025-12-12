use crate::core::{camera, scene};

pub struct Render {
    pub width: u32,
    pub samples: u32,
    pub depth: u32,
    pub camera: camera::Camera,
    pub scene: scene::Scene,
}
