/// Camera module defining the `Camera` struct and related functionality.

use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::core::ray;
use crate::math::vec;

/// Parameters used to build a [`Camera`].
#[derive(Debug, Clone, Copy)]
pub struct CameraConfig {
    /// Camera position.
    pub origin: vec::Vec3,
    /// Point to aim the camera at.
    pub look_at: vec::Vec3,
    /// Up vector used to orient the camera.
    pub up: vec::Vec3,
    /// Image aspect ratio (width / height).
    pub aspect_ratio: f32,
    /// Height of the viewport in world space.
    pub viewport_height: f32,
    /// Distance from camera origin to viewport plane.
    pub focal_length: f32,
    /// Lens aperture size controlling depth of field blur.
    pub aperture: f32,
    /// Vertical field of view in degrees.
    pub vertical_fov: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Ray generator that maps screen coordinates to rays in world space.
pub struct Camera {
    pub origin: vec::Vec3,
    pub lower_left_corner: vec::Vec3,
    pub horizontal: vec::Vec3,
    pub vertical: vec::Vec3,
    pub up: vec::Vec3,
    pub u: vec::Vec3,
    pub v: vec::Vec3,
    pub w: vec::Vec3,
    pub focal_length: f32,
    pub aperture: f32,
    pub vertical_fov: f32,
    pub aspect_ratio: f32,
}

impl Camera {
    /// Creates a camera with sensible defaults (16:9, 90° FOV).
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

    /// Constructs a camera from a full configuration.
    pub fn with_config(config: CameraConfig) -> Self {
        let theta = config.vertical_fov.to_radians();
        let half_height = (theta / 2.0).tan();
        let half_width = config.aspect_ratio * half_height;
        let focus_dist = config.focal_length;

        let w = (config.origin - config.look_at).normalize();
        let u = config.up.cross(&w).normalize();
        let v = w.cross(&u);

        let horizontal = u * half_width * 2.0 * focus_dist;
        let vertical = v * half_height * 2.0 * focus_dist;
        let lower_left_corner =
            config.origin - (horizontal / 2.0) - (vertical / 2.0) - w * focus_dist;

        let camera = Camera {
            origin: config.origin,
            focal_length: config.focal_length,
            aperture: config.aperture,
            vertical_fov: config.vertical_fov,
            aspect_ratio: config.aspect_ratio,
            up: config.up,
            u,
            v,
            w,
            lower_left_corner,
            horizontal,
            vertical,
        };

        camera
    }

    /// Re-aims the camera at a new target while preserving viewport size.
    pub fn look_at(&mut self, val: &vec::Vec3) {
        let w = (self.origin - *val).normalize();
        let u = self.up.cross(&w).normalize();
        let v = w.cross(&u);

        let horizontal_len = self.horizontal.length();
        let vertical_len = self.vertical.length();

        self.horizontal = u * horizontal_len;
        self.vertical = v * vertical_len;
        self.lower_left_corner =
            self.origin - (self.horizontal / 2.0) - (self.vertical / 2.0) - w * self.focal_length;
    }

    /// Generates a ray through normalized viewport coordinates (`u`, `v`).
    pub fn get_ray(&self, rng: &mut rand::rngs::ThreadRng, u: f32, v: f32) -> ray::Ray {
        let lens_radius = self.aperture / 2.0;
        let rd = lens_radius * vec::random_in_unit_disk(rng);
        let offset = self.u * rd.x + self.v * rd.y;
        let ray_time = rng.random::<f64>();

        ray::Ray {
            origin: self.origin + offset,
            direction: self.lower_left_corner + u * self.horizontal + v * self.vertical
                - self.origin
                - offset,
            time: ray_time,
        }
    }
}
