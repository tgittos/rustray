//! Ray representation used for casting through the scene.
use serde::{Deserialize, Serialize};

use crate::core::vec;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
/// A half-infinite line defined by an origin and direction, with time parameter.
pub struct Ray {
    pub origin: vec::Vec3,
    pub direction: vec::Vec3,
    pub time: f64,
}

impl Ray {
    /// Creates a new ray from an origin and direction, with an optional time parameter.
    pub fn new(origin: &vec::Vec3, direction: &vec::Vec3, time: Option<f64>) -> Self {
        Ray {
            origin: *origin,
            direction: *direction,
            time: time.unwrap_or(0.0),
        }
    }

    /// Returns the point at parameter `t` along the ray.
    pub fn point_at(&self, t: f32) -> vec::Vec3 {
        self.origin + self.direction * t
    }
}
