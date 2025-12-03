use crate::core::vec;
use crate::core::ray;
use crate::traits::hittable::HitRecord;
use crate::traits::hittable::Hittable;

pub struct Scene {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl Scene {
    pub fn new() -> Self {
        Scene { objects: Vec::new() }
    }

    pub fn add_object(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for Scene {
    fn hit<'a>(&'a self, ray: &ray::Ray, t_min: f32, t_max: f32) -> Option<HitRecord<'a>> {
        let mut closest_so_far = t_max;
        let mut hit_record: Option<HitRecord<'a>> = None;

        for object in &self.objects {
            if let Some(temp_record) = object.hit(ray, t_min, closest_so_far) {
                closest_so_far = temp_record.t;
                hit_record = Some(temp_record);
            }
        }

        hit_record
    }
}
