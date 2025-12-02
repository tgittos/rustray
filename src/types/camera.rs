use crate::Vec3;
use crate::Ray;

#[derive(Debug)]
pub struct Camera {
    pub origin: crate::Vec3,
    pub lower_left_corner: crate::Vec3,
    pub horizontal: crate::Vec3,
    pub vertical: crate::Vec3,
}

impl Camera {
    pub fn new() -> Self {
        let aspect_ratio = 16.0 / 9.0;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let origin = crate::Vec3::new(0.0, 0.0, 0.0);
        let horizontal = crate::Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = crate::Vec3::new(0.0, viewport_height, 0.0);
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - crate::Vec3::new(0.0, 0.0, focal_length);

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> crate::Ray {
        let direction = self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin;
        crate::Ray::new(&self.origin, &direction)
    }
}