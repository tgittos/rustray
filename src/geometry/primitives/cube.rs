use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::core::{bbox, ray};
use crate::math::{pdf, vec};
use crate::traits::hittable;
use crate::traits::hittable::Hittable;

use super::quad;

pub struct CubePDF<'a> {
    cube: &'a Cube,
    origin: vec::Point3,
    time: f64,
}

impl<'a> CubePDF<'a> {
    pub fn new(cube: &'a Cube, origin: vec::Point3, time: f64) -> Self {
        CubePDF { cube, origin, time }
    }
}

impl pdf::PDF for CubePDF<'_> {
    fn value(&self, direction: vec::Vec3) -> f32 {
        let ray = ray::Ray::new(&self.origin, &direction, Some(self.time));
        let Some(hit) = self.cube.hit(&ray, 0.001, f32::MAX) else {
            return 0.0;
        };
        let dims = self.cube.max - self.cube.min;
        let area = 2.0 * (dims.x * dims.y + dims.x * dims.z + dims.y * dims.z);
        let direction_len_sq = direction.squared_length();
        if direction_len_sq <= f32::EPSILON {
            return 0.0;
        }
        let distance_squared = hit.t * hit.t * direction_len_sq;
        let cosine = (direction.dot(&hit.normal) / direction_len_sq.sqrt()).abs();
        if cosine <= 0.0 {
            return 0.0;
        }
        distance_squared / (cosine * area)
    }

    fn generate(&self, rng: &mut rand::rngs::ThreadRng) -> vec::Vec3 {
        let mut areas = [0.0_f32; 6];
        let mut total_area = 0.0_f32;
        for (idx, face) in self.cube.faces.iter().enumerate() {
            let area = face.u.cross(&face.v).length();
            areas[idx] = area;
            total_area += area;
        }

        let mut pick = rng.random::<f32>() * total_area;
        let mut face_index = 0;
        for (idx, area) in areas.iter().enumerate() {
            if pick <= *area {
                face_index = idx;
                break;
            }
            pick -= area;
        }

        let face = &self.cube.faces[face_index];
        let r1: f32 = rng.random::<f32>();
        let r2: f32 = rng.random::<f32>();
        let point = face.q + face.u * r1 + face.v * r2;
        point - self.origin
    }
}

/// Axis-aligned cube assembled from six quads.
#[derive(Clone, Serialize)]
pub struct Cube {
    pub min: vec::Vec3,
    pub max: vec::Vec3,

    #[serde(skip)]
    faces: [quad::Quad; 6],

    #[serde(skip)]
    bbox: bbox::BBox,
}

impl Cube {
    pub fn new(min: vec::Vec3, max: vec::Vec3) -> Self {
        let min_point = vec::Vec3::new(min.x.min(max.x), min.y.min(max.y), min.z.min(max.z));
        let max_point = vec::Vec3::new(min.x.max(max.x), min.y.max(max.y), min.z.max(max.z));
        let faces = Cube::build_faces(&min_point, &max_point);
        let bbox = bbox::BBox::bounding(min_point, max_point);

        Cube {
            min: min_point,
            max: max_point,
            faces,
            bbox,
        }
    }

    fn build_faces(min: &vec::Vec3, max: &vec::Vec3) -> [quad::Quad; 6] {
        let dx = max.x - min.x;
        let dy = max.y - min.y;
        let dz = max.z - min.z;

        [
            // +Z face
            quad::Quad::new(
                vec::Vec3::new(min.x, min.y, max.z),
                vec::Vec3::new(dx, 0.0, 0.0),
                vec::Vec3::new(0.0, dy, 0.0),
            ),
            // -Z face
            quad::Quad::new(
                vec::Vec3::new(max.x, min.y, min.z),
                vec::Vec3::new(-dx, 0.0, 0.0),
                vec::Vec3::new(0.0, dy, 0.0),
            ),
            // -X face
            quad::Quad::new(
                vec::Vec3::new(min.x, min.y, min.z),
                vec::Vec3::new(0.0, 0.0, dz),
                vec::Vec3::new(0.0, dy, 0.0),
            ),
            // +X face
            quad::Quad::new(
                vec::Vec3::new(max.x, min.y, max.z),
                vec::Vec3::new(0.0, 0.0, -dz),
                vec::Vec3::new(0.0, dy, 0.0),
            ),
            // +Y face
            quad::Quad::new(
                vec::Vec3::new(min.x, max.y, max.z),
                vec::Vec3::new(dx, 0.0, 0.0),
                vec::Vec3::new(0.0, 0.0, -dz),
            ),
            // -Y face
            quad::Quad::new(
                vec::Vec3::new(min.x, min.y, min.z),
                vec::Vec3::new(dx, 0.0, 0.0),
                vec::Vec3::new(0.0, 0.0, dz),
            ),
        ]
    }
}

impl<'de> Deserialize<'de> for Cube {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct CubeData {
            min: vec::Vec3,
            max: vec::Vec3,
        }

        let data = CubeData::deserialize(deserializer)?;
        Ok(Cube::new(data.min, data.max))
    }
}

impl hittable::Hittable for Cube {
    fn hit(&self, ray: &ray::Ray, t_min: f32, t_max: f32) -> Option<hittable::Hit> {
        if !self.bbox.hit(ray, t_min, t_max) {
            return None;
        }

        let mut closest = t_max;
        let mut hit_record: Option<hittable::Hit> = None;

        for face in self.faces.iter() {
            if let Some(hit) = face.hit(ray, t_min, closest) {
                closest = hit.t;
                hit_record = Some(hit);
            }
        }

        hit_record
    }

    fn bounding_box(&self) -> bbox::BBox {
        self.bbox
    }

    fn get_pdf(&self, origin: &vec::Point3, time: f64) -> Box<dyn pdf::PDF + Send + Sync + '_> {
        Box::new(CubePDF::new(self, *origin, time))
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
