//! Ray representation used for casting through the scene.
use crate::core::vec;

#[derive(Clone, Copy)]
/// A half-infinite line defined by an origin and direction.
pub struct Ray {
    pub origin: vec::Vec3,
    pub direction: vec::Vec3,
}

impl Ray {
    /// Creates a new ray from an origin and direction.
    pub fn new(origin: &vec::Vec3, direction: &vec::Vec3) -> Self {
        Ray {
            origin: *origin,
            direction: *direction,
        }
    }

    /// Returns the point at parameter `t` along the ray.
    pub fn point_at(&self, t: f32) -> vec::Vec3 {
        self.origin + self.direction * t
    }
}
