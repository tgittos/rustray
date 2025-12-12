use serde::{Deserialize, Serialize};

use crate::core::{bbox, ray};
use crate::math::{mat, vec};

#[derive(Clone, Serialize, Deserialize)]
pub enum Transform {
    Rotate(mat::Mat3),
    Translate(vec::Vec3),
    Scale(vec::Vec3),
    Move {
        start: vec::Vec3,
        end: vec::Vec3,
        time_start: f64,
        time_end: f64,
    },
}

impl Transform {
    pub fn apply_point(&self, point: &vec::Vec3, time: f64) -> vec::Vec3 {
        match self {
            Transform::Rotate(mat) => mat * *point,
            Transform::Translate(offset) => *point + *offset,
            Transform::Move {
                start,
                end,
                time_start,
                time_end,
            } => *point + Self::move_offset(start, end, *time_start, *time_end, time),
            Transform::Scale(factors) => vec::Vec3 {
                x: point.x * factors.x,
                y: point.y * factors.y,
                z: point.z * factors.z,
            },
        }
    }

    pub fn apply_normal(&self, normal: &vec::Vec3, _time: f64) -> vec::Vec3 {
        match self {
            Transform::Rotate(mat) => vec::unit_vector(&(mat * *normal)),
            Transform::Translate(_) => *normal,
            Transform::Move { .. } => *normal,
            Transform::Scale(factors) => vec::unit_vector(&vec::Vec3 {
                x: normal.x / factors.x,
                y: normal.y / factors.y,
                z: normal.z / factors.z,
            }),
        }
    }

    pub fn apply_inverse(&self, ray: &ray::Ray) -> ray::Ray {
        match self {
            Transform::Rotate(mat) => {
                // Assuming mat is orthogonal, its inverse is its transpose
                let transposed = mat.transpose();
                ray::Ray {
                    origin: transposed * ray.origin,
                    direction: transposed * ray.direction,
                    time: ray.time,
                }
            }
            Transform::Translate(offset) => ray::Ray {
                origin: ray.origin - *offset,
                direction: ray.direction,
                time: ray.time,
            },
            Transform::Scale(factors) => ray::Ray {
                origin: vec::Vec3 {
                    x: ray.origin.x / factors.x,
                    y: ray.origin.y / factors.y,
                    z: ray.origin.z / factors.z,
                },
                direction: vec::Vec3 {
                    x: ray.direction.x / factors.x,
                    y: ray.direction.y / factors.y,
                    z: ray.direction.z / factors.z,
                },
                time: ray.time,
            },
            Transform::Move {
                start,
                end,
                time_start,
                time_end,
            } => {
                let offset = Self::move_offset(start, end, *time_start, *time_end, ray.time);
                ray::Ray {
                    origin: ray.origin - offset,
                    direction: ray.direction,
                    time: ray.time,
                }
            }
        }
    }

    pub fn apply_bbox(&self, bbox: &bbox::BBox) -> bbox::BBox {
        match self {
            Transform::Translate(offset) => bbox::BBox::bounding(
                vec::Vec3::new(
                    bbox.x.min + offset.x,
                    bbox.y.min + offset.y,
                    bbox.z.min + offset.z,
                ),
                vec::Vec3::new(
                    bbox.x.max + offset.x,
                    bbox.y.max + offset.y,
                    bbox.z.max + offset.z,
                ),
            ),
            Transform::Scale(factors) => {
                let (x0, x1) = (bbox.x.min * factors.x, bbox.x.max * factors.x);
                let (y0, y1) = (bbox.y.min * factors.y, bbox.y.max * factors.y);
                let (z0, z1) = (bbox.z.min * factors.z, bbox.z.max * factors.z);

                bbox::BBox::bounding(
                    vec::Vec3::new(x0.min(x1), y0.min(y1), z0.min(z1)),
                    vec::Vec3::new(x0.max(x1), y0.max(y1), z0.max(z1)),
                )
            }
            Transform::Rotate(mat) => {
                let corners = [
                    vec::Vec3::new(bbox.x.min, bbox.y.min, bbox.z.min),
                    vec::Vec3::new(bbox.x.min, bbox.y.min, bbox.z.max),
                    vec::Vec3::new(bbox.x.min, bbox.y.max, bbox.z.min),
                    vec::Vec3::new(bbox.x.min, bbox.y.max, bbox.z.max),
                    vec::Vec3::new(bbox.x.max, bbox.y.min, bbox.z.min),
                    vec::Vec3::new(bbox.x.max, bbox.y.min, bbox.z.max),
                    vec::Vec3::new(bbox.x.max, bbox.y.max, bbox.z.min),
                    vec::Vec3::new(bbox.x.max, bbox.y.max, bbox.z.max),
                ];
                let rotated = corners.map(|corner| mat * corner);
                let mut min = rotated[0];
                let mut max = rotated[0];
                for point in rotated.iter().skip(1) {
                    min =
                        vec::Vec3::new(min.x.min(point.x), min.y.min(point.y), min.z.min(point.z));
                    max =
                        vec::Vec3::new(max.x.max(point.x), max.y.max(point.y), max.z.max(point.z));
                }
                bbox::BBox::bounding(min, max)
            }
            Transform::Move {
                start,
                end,
                time_start: _,
                time_end: _,
            } => {
                let moved_min = bbox::BBox::bounding(
                    vec::Vec3::new(
                        bbox.x.min + start.x,
                        bbox.y.min + start.y,
                        bbox.z.min + start.z,
                    ),
                    vec::Vec3::new(
                        bbox.x.max + start.x,
                        bbox.y.max + start.y,
                        bbox.z.max + start.z,
                    ),
                );
                let moved_max = bbox::BBox::bounding(
                    vec::Vec3::new(bbox.x.min + end.x, bbox.y.min + end.y, bbox.z.min + end.z),
                    vec::Vec3::new(bbox.x.max + end.x, bbox.y.max + end.y, bbox.z.max + end.z),
                );
                moved_min.union(&moved_max)
            }
        }
    }

    fn move_offset(
        start: &vec::Vec3,
        end: &vec::Vec3,
        time_start: f64,
        time_end: f64,
        time: f64,
    ) -> vec::Vec3 {
        let duration = (time_end - time_start).max(f64::EPSILON);
        let lerp_t = ((time - time_start) / duration).clamp(0.0, 1.0) as f32;
        *start + (*end - *start) * lerp_t
    }
}
