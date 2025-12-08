/// An axis-aligned bounding box.
use std::mem;

use crate::core::{interval, ray, vec};

#[derive(Clone, Copy, Debug)]
pub struct BBox {
    pub x: interval::Interval,
    pub y: interval::Interval,
    pub z: interval::Interval,
}

impl BBox {
    pub fn new(x: interval::Interval, y: interval::Interval, z: interval::Interval) -> Self {
        let inst = BBox { x, y, z };
        inst.pad_to_min(0.0001);
        inst
    }

    pub fn bounding(min: vec::Point3, max: vec::Point3) -> Self {
        BBox {
            x: interval::Interval::new(min.x, max.x),
            y: interval::Interval::new(min.y, max.y),
            z: interval::Interval::new(min.z, max.z),
        }
    }

    pub fn union(&self, other: &BBox) -> BBox {
        BBox {
            x: interval::Interval::new(self.x.min.min(other.x.min), self.x.max.max(other.x.max)),
            y: interval::Interval::new(self.y.min.min(other.y.min), self.y.max.max(other.y.max)),
            z: interval::Interval::new(self.z.min.min(other.z.min), self.z.max.max(other.z.max)),
        }
    }

    pub fn axis(&self, axis: usize) -> &interval::Interval {
        match axis {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Invalid axis index"),
        }
    }

    pub fn longest_axis(&self) -> usize {
        let x_length = self.x.length();
        let y_length = self.y.length();
        let z_length = self.z.length();

        if x_length > y_length && x_length > z_length {
            0
        } else if y_length > z_length {
            1
        } else {
            2
        }
    }

    pub fn pad_to_min(&self, delta: f32) {
        if self.x.length() < delta {
            self.x.expand(delta);
        }
        if self.y.length() < delta {
            self.y.expand(delta);
        }
        if self.z.length() < delta {
            self.z.expand(delta);
        }
    }

    pub fn hit(&self, ray: &ray::Ray, t_min: f32, t_max: f32) -> bool {
        let inv_dir = vec::Vec3::new(
            1.0 / ray.direction.x,
            1.0 / ray.direction.y,
            1.0 / ray.direction.z,
        );

        let mut t0 = (self.x.min - ray.origin.x) * inv_dir.x;
        let mut t1 = (self.x.max - ray.origin.x) * inv_dir.x;

        if inv_dir.x < 0.0 {
            mem::swap(&mut t0, &mut t1);
        }

        let mut t_min = t0.max(t_min);
        let mut t_max = t1.min(t_max);

        t0 = (self.y.min - ray.origin.y) * inv_dir.y;
        t1 = (self.y.max - ray.origin.y) * inv_dir.y;

        if inv_dir.y < 0.0 {
            mem::swap(&mut t0, &mut t1);
        }

        t_min = t0.max(t_min);
        t_max = t1.min(t_max);

        t0 = (self.z.min - ray.origin.z) * inv_dir.z;
        t1 = (self.z.max - ray.origin.z) * inv_dir.z;

        if inv_dir.z < 0.0 {
            mem::swap(&mut t0, &mut t1);
        }

        t_min = t0.max(t_min);
        t_max = t1.min(t_max);

        t_max > t_min
    }
}
