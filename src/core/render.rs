use serde::{Deserialize, Serialize};

use crate::core::{camera, scene};

#[derive(Serialize, Deserialize)]
pub struct Render {
    pub width: u32,
    pub samples: u32,
    pub depth: u32,
    pub camera: camera::Camera,
    pub scene: scene::Scene,
}
