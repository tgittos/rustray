use std::mem;

use crate::core::{ray, vec};

pub struct BBox {
    pub min: vec::Vec3,
    pub max: vec::Vec3,
}

impl BBox {
    pub fn new(min: vec::Vec3, max: vec::Vec3) -> Self {
        BBox { min, max }
    }

    pub fn hit(&self, ray: &ray::Ray, t_min: f32, t_max: f32) -> bool {
        for a in 0..3 {
            let inv_d = 1.0 / ray.direction.get(a);
            let mut t0 = (self.min.get(a) - ray.origin.get(a)) * inv_d;
            let mut t1 = (self.max.get(a) - ray.origin.get(a)) * inv_d;
            if inv_d < 0.0 {
                mem::swap(&mut t0, &mut t1);
            }
            let t_min = if t0 > t_min { t0 } else { t_min };
            let t_max = if t1 < t_max { t1 } else { t_max };
            if t_max <= t_min {
                return false;
            }
        }
        true
    }
}
