use crate::core::vec;
use crate::core::ray;

pub struct CameraConfig {
    pub origin: vec::Vec3,
    pub look_at: vec::Vec3,
    pub up: vec::Vec3,
    pub aspect_ratio: f32,
    pub viewport_height: f32,
    pub focal_length: f32,
    pub aperture: f32,
    pub vertical_fov: f32,
}

#[derive(Debug)]
pub struct Camera {
    pub origin: vec::Vec3,
    pub lower_left_corner: vec::Vec3,
    pub horizontal: vec::Vec3,
    pub vertical: vec::Vec3,
    pub up: vec::Vec3,
    pub focal_length: f32,
    pub aperture: f32,
    pub vertical_fov: f32,
}

impl Camera {
    pub fn new() -> Self {
        Camera::with_config(CameraConfig {
            origin: vec::Vec3::new(0.0, 0.0, 0.0),
            look_at: vec::Vec3::new(0.0, 0.0, -1.0),
            up: vec::Vec3::new(0.0, 1.0, 0.0),
            aspect_ratio: 16.0 / 9.0,
            viewport_height: 2.0,
            focal_length: 1.0,
            vertical_fov: 90.0,
            aperture: 0.0,
        })
    }

    pub fn with_config(config: CameraConfig) -> Self {
        let theta = config.vertical_fov.to_radians();
        let half_height = (theta / 2.0).tan();
        let half_width = config.aspect_ratio * half_height;

        let w = (config.origin - config.look_at).normalize();
        let u = config.up.cross(&w).normalize();
        let v = w.cross(&u);

        let horizontal = u * half_width * 2.0;
        let vertical = v * half_height * 2.0;
        let lower_left_corner = config.origin - half_width * config.focal_length * u - half_height * config.focal_length * v - w * config.focal_length;

        let camera = Camera {
            origin: config.origin,
            focal_length: config.focal_length,
            aperture: config.aperture,
            vertical_fov: config.vertical_fov,
            up: config.up,
            lower_left_corner,
            horizontal,
            vertical,
        };

        camera
    }

    pub fn look_at(&mut self, val: &vec::Vec3) {
        let w = (self.origin - *val).normalize();
        let u = self.up.cross(&w).normalize();
        let v = w.cross(&u);

        let horizontal_len = self.horizontal.length();
        let vertical_len = self.vertical.length();

        self.horizontal = u * horizontal_len;
        self.vertical = v * vertical_len;
        self.lower_left_corner = self.origin - (self.horizontal / 2.0) - (self.vertical / 2.0) - w * self.focal_length;
    }

    pub fn get_ray(&self, u: f32, v: f32) -> ray::Ray {
        let lens_radius = self.aperture / 2.0;
        let rd = lens_radius * vec::random_in_unit_disk(&mut rand::rng());
        let offset = self.up.cross(&((self.horizontal).normalize())) * rd.x + self.up.cross(&((self.vertical).normalize())) * rd.y;
        ray::Ray {
            origin: self.origin + offset,
            direction: self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin - offset,
        }
    }
}
