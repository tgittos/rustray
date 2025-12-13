use crate::math::vec;

pub struct ONB {
    pub u: vec::Vec3,
    pub v: vec::Vec3,
    pub w: vec::Vec3,
}

impl ONB {
    /// Builds an orthonormal basis from the given normal vector.
    pub fn build_from_w(n: &vec::Vec3) -> Self {
        let w = vec::unit_vector(n);
        let a = if w.x.abs() > 0.9 {
            vec::Vec3::new(0.0, 1.0, 0.0)
        } else {
            vec::Vec3::new(1.0, 0.0, 0.0)
        };
        let v = w.cross(&a).normalize();
        let u = w.cross(&v);
        ONB { u, v, w }
    }

    /// Converts local coordinates to world coordinates.
    pub fn local(&self, a: &vec::Vec3) -> vec::Vec3 {
        self.u * a.x + self.v * a.y + self.w * a.z
    }
}
