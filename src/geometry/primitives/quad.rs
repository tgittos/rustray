use serde::{Deserialize, Serialize};

use crate::core::{bbox, ray};
use crate::math::vec;
use crate::traits::hittable;

#[derive(Serialize)]
pub struct Quad {
    pub q: vec::Point3,
    pub u: vec::Vec3,
    pub v: vec::Vec3,
    pub w: vec::Vec3,

    #[serde(skip)]
    bbox: bbox::BBox,

    #[serde(skip)]
    normal: vec::Vec3,

    #[serde(skip)]
    d: f32,
}

impl Quad {
    pub fn new(q: vec::Point3, u: vec::Vec3, v: vec::Vec3) -> Self {
        let bbox = bbox::BBox::bounding(q, q + u + v).union(&bbox::BBox::bounding(q + u, q + v));
        let normal = u.cross(&v).normalize();
        let d = normal.dot(&(q as vec::Vec3));
        let w = normal / normal.dot(&normal);
        Quad {
            q,
            u,
            v,
            bbox,
            normal,
            d,
            w,
        }
    }

    fn get_uv(&self, point: &vec::Point3) -> (f32, f32) {
        let w = *point - self.q;
        let u_len_sq = self.u.dot(&self.u);
        let v_len_sq = self.v.dot(&self.v);
        let u_coord = w.dot(&self.u) / u_len_sq;
        let v_coord = w.dot(&self.v) / v_len_sq;
        (u_coord, v_coord)
    }
}

impl Clone for Quad {
    fn clone(&self) -> Self {
        Quad::new(self.q, self.u, self.v)
    }
}

impl<'de> Deserialize<'de> for Quad {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct QuadData {
            q: vec::Point3,
            u: vec::Vec3,
            v: vec::Vec3,
        }

        let data = QuadData::deserialize(deserializer)?;
        let bbox = bbox::BBox::bounding(data.q, data.q + data.u + data.v)
            .union(&bbox::BBox::bounding(data.q + data.u, data.q + data.v));
        let normal = data.u.cross(&data.v).normalize();
        let d = normal.dot(&data.q);
        let w = normal / normal.dot(&normal);

        Ok(Quad {
            q: data.q,
            u: data.u,
            v: data.v,
            w,
            bbox,
            normal,
            d,
        })
    }
}

impl hittable::Hittable for Quad {
    fn hit(&self, ray: &ray::Ray, t_min: f32, t_max: f32) -> Option<hittable::Hit> {
        let denom = self.normal.dot(&ray.direction);
        if denom.abs() < 1e-6 {
            return None;
        }

        let t = (self.d - self.normal.dot(&(ray.origin as vec::Vec3))) / denom;
        if t < t_min || t > t_max {
            return None;
        }

        let p = ray.point_at(t);
        let w = p - self.q;

        let u_dot_u = self.u.dot(&self.u);
        let u_dot_v = self.u.dot(&self.v);
        let v_dot_v = self.v.dot(&self.v);
        let w_dot_u = w.dot(&self.u);
        let w_dot_v = w.dot(&self.v);

        let denom_quad = u_dot_u * v_dot_v - u_dot_v * u_dot_v;
        let s = (v_dot_v * w_dot_u - u_dot_v * w_dot_v) / denom_quad;
        let t_param = (u_dot_u * w_dot_v - u_dot_v * w_dot_u) / denom_quad;

        if s < 0.0 || s > 1.0 || t_param < 0.0 || t_param > 1.0 {
            return None;
        }

        let (u_coord, v_coord) = self.get_uv(&p);

        Some(hittable::Hit {
            t,
            point: p,
            ray: ray.clone(),
            normal: self.normal,
            u: u_coord,
            v: v_coord,
        })
    }

    fn bounding_box(&self) -> bbox::BBox {
        self.bbox
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
