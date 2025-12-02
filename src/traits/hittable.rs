pub struct HitRecord {
    pub t: f32,
    pub point: crate::Vec3,
    pub normal: crate::Vec3,
}

pub trait Hittable {
    fn hit(&self, ray: &crate::Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}