use crate::core::vec;

#[derive(Clone, Copy)]
pub struct Ray {
    pub origin: vec::Vec3,
    pub direction: vec::Vec3,
}

impl Ray {
    pub fn new(origin: &vec::Vec3, direction: &vec::Vec3) -> Self {
        Ray { origin: *origin, direction: *direction }
    }

    pub fn point_at(&self, t: f32) -> vec::Vec3 {
        self.origin + self.direction * t
    }
}