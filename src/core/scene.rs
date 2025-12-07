//! Scene container that stores renderable objects and routes ray intersections.
use serde::{Deserialize, Serialize};
use std::{fs, path::Path, time};

use crate::core::{bvh, camera, ray, vec};
use crate::materials::{dielectric, lambertian, metallic};
use crate::primitives::{skybox, sphere};
use crate::textures::{checker, color};
use crate::traits::hittable;
use crate::traits::renderable;
use crate::traits::renderable::Renderable;
use crate::traits::{sampleable, texturable};
use crate::utils::stats;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct RenderSettings {
    pub width: u32,
    #[serde(alias = "ar")]
    pub aspect_ratio: f32,
    pub samples: u32,
    #[serde(rename = "depth")]
    pub max_depth: u32,
}

impl RenderSettings {
    pub fn aspect_ratio(&self) -> f32 {
        if self.aspect_ratio <= 0.0 {
            16.0 / 9.0
        } else {
            self.aspect_ratio
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneDocument {
    pub render: RenderSettings,
    pub camera: camera::CameraConfig,
    pub objects: Vec<SceneObject>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Motion {
    pub end: vec::Vec3,
    #[serde(default = "default_time_start")]
    pub time_start: f64,
    #[serde(default = "default_time_end")]
    pub time_end: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Texture {
    Color {
        albedo: vec::Vec3,
    },
    Checker {
        color1: vec::Vec3,
        color2: vec::Vec3,
        scale: f32,
    },
}

impl Texture {
    fn to_texturable(&self) -> Box<dyn texturable::Texturable> {
        match self {
            Texture::Color { albedo } => Box::new(color::ColorTexture::new(*albedo)),
            Texture::Checker {
                color1,
                color2,
                scale,
            } => Box::new(checker::CheckerTexture::new(*color1, *color2, *scale)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Material {
    Lambertian { texture: Texture },
    Metallic { albedo: vec::Vec3, roughness: f32 },
    Dielectric { refractive_index: f32 },
}

impl Material {
    fn to_sampleable(&self) -> Box<dyn sampleable::Sampleable> {
        match self {
            Material::Lambertian { texture } => {
                Box::new(lambertian::Diffuse::new(texture.to_texturable()))
            }
            Material::Metallic { albedo, roughness } => {
                Box::new(metallic::Metallic::new(albedo, *roughness))
            }
            Material::Dielectric { refractive_index } => {
                Box::new(dielectric::Dielectric::new(*refractive_index))
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum SceneObject {
    Sphere {
        center: vec::Vec3,
        radius: f32,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        motion: Option<Motion>,
        material: Material,
    },
    Skybox {
        top_color: vec::Vec3,
        bottom_color: vec::Vec3,
    },
}

impl SceneObject {
    fn to_renderable(&self) -> Box<dyn renderable::Renderable> {
        match self {
            SceneObject::Sphere {
                center,
                radius,
                motion,
                material,
            } => {
                let hittable: Box<dyn hittable::Hittable> = if let Some(motion) = motion {
                    Box::new(sphere::Sphere::moving(
                        center,
                        &motion.end,
                        motion.time_start,
                        motion.time_end,
                        *radius,
                    ))
                } else {
                    Box::new(sphere::Sphere::new(center, *radius))
                };
                let sampleable = material.to_sampleable();
                Box::new(renderable::create_renderable(hittable, sampleable))
            }
            SceneObject::Skybox {
                top_color,
                bottom_color,
            } => {
                let primitive = skybox::Skybox::new(top_color, bottom_color);
                Box::new(renderable::create_renderable(
                    Box::new(primitive),
                    Box::new(primitive),
                ))
            }
        }
    }
}

#[derive(Debug)]
pub enum SceneIoError {
    Io(std::io::Error),
    Parse(toml::de::Error),
    Serialize(toml::ser::Error),
    MissingDescriptor,
}

impl std::fmt::Display for SceneIoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SceneIoError::Io(err) => write!(f, "scene I/O error: {}", err),
            SceneIoError::Parse(err) => write!(f, "scene parse error: {}", err),
            SceneIoError::Serialize(err) => write!(f, "scene serialize error: {}", err),
            SceneIoError::MissingDescriptor => {
                write!(f, "scene missing serializable descriptor")
            }
        }
    }
}

impl std::error::Error for SceneIoError {}

impl From<std::io::Error> for SceneIoError {
    fn from(value: std::io::Error) -> Self {
        SceneIoError::Io(value)
    }
}

impl From<toml::de::Error> for SceneIoError {
    fn from(value: toml::de::Error) -> Self {
        SceneIoError::Parse(value)
    }
}

impl From<toml::ser::Error> for SceneIoError {
    fn from(value: toml::ser::Error) -> Self {
        SceneIoError::Serialize(value)
    }
}

impl SceneDocument {
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, SceneIoError> {
        let contents = fs::read_to_string(path)?;
        Ok(toml::from_str(&contents)?)
    }

    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), SceneIoError> {
        let path = path.as_ref();

        if let Some(parent) = path.parent() {
            if !parent.as_os_str().is_empty() {
                fs::create_dir_all(parent)?;
            }
        }

        let serialized = toml::to_string_pretty(self)?;
        fs::write(path, serialized)?;
        Ok(())
    }
}

fn default_time_start() -> f64 {
    0.0
}

fn default_time_end() -> f64 {
    1.0
}

/// Collection of renderable objects making up the world.
pub struct Scene {
    pub objects: renderable::RenderableList,
    pub bvh: Option<bvh::Bvh>,
    descriptor: Option<SceneDocument>,
}

impl Scene {
    /// Creates an empty scene.
    pub fn new() -> Self {
        Scene {
            objects: renderable::RenderableList::new(),
            bvh: None,
            descriptor: None,
        }
    }

    pub fn with_descriptor(descriptor: SceneDocument) -> Self {
        Scene {
            objects: renderable::RenderableList::new(),
            bvh: None,
            descriptor: Some(descriptor),
        }
    }

    pub fn descriptor(&self) -> Option<&SceneDocument> {
        self.descriptor.as_ref()
    }

    pub fn from_document(
        document: SceneDocument,
        rng: &mut rand::rngs::ThreadRng,
    ) -> (Self, camera::Camera) {
        let mut scene = Scene::new();

        for object in document.objects.iter() {
            scene.add_object(object.to_renderable());
        }

        scene.build_bvh(rng);
        scene.descriptor = Some(document.clone());

        let camera = camera::Camera::with_config(document.camera);

        (scene, camera)
    }

    pub fn load_from_file<P: AsRef<Path>>(
        path: P,
        rng: &mut rand::rngs::ThreadRng,
    ) -> Result<(Self, camera::Camera, RenderSettings), SceneIoError> {
        let document = SceneDocument::load_from_file(path)?;
        let render = document.render;
        let (scene, camera) = Scene::from_document(document, rng);
        Ok((scene, camera, render))
    }

    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), SceneIoError> {
        let descriptor = self
            .descriptor
            .as_ref()
            .ok_or(SceneIoError::MissingDescriptor)?;
        descriptor.save_to_file(path)
    }

    /// Adds a renderable object to the scene.
    pub fn add_object(&mut self, object: Box<dyn renderable::Renderable>) {
        if self.descriptor.is_some() {
            self.descriptor = None;
        }

        if let Some(bvh) = self.bvh.take() {
            // BVH already exists; flatten it back into the list before adding more.
            let renderables = bvh.into_renderables();
            self.objects = renderable::RenderableList::new();
            renderables.into_iter().for_each(|renderable| {
                self.objects.add(renderable);
            });
        }
        self.objects.add(object);
    }

    /// Builds the BVH from all current renderables.
    pub fn build_bvh(&mut self, rng: &mut rand::rngs::ThreadRng) {
        if self.objects.objects.is_empty() {
            self.bvh = None;
            return;
        }

        let renderables = std::mem::take(&mut self.objects.objects);
        let bvh_root = bvh::Bvh::new(rng, renderables);
        self.objects.bbox = bvh_root.bounding_box().clone();
        self.bvh = Some(bvh_root);
    }
}

/// Finds the closest intersection among scene objects.
fn scene_hit<'a>(
    ray: &ray::Ray,
    scene: &'a Scene,
    t_min: f32,
    t_max: f32,
) -> Option<hittable::HitRecord<'a>> {
    if let Some(bvh) = &scene.bvh {
        return bvh.hit(ray, t_min, t_max);
    }

    let mut closest_so_far = t_max;
    let mut hit_record: Option<hittable::HitRecord> = None;

    if !scene.objects.bbox.hit(ray, t_min, t_max) {
        return None;
    }

    for object in scene.objects.objects.iter() {
        let hit_start = time::Instant::now();
        if let Some(temp_record) = object.hit(ray, t_min, closest_so_far) {
            stats::add_hit_stat(stats::Stat::new(stats::SCENE_HIT, hit_start.elapsed()));

            closest_so_far = temp_record.hit.t;
            hit_record = Some(temp_record);
        }
    }

    hit_record
}

/// Delegates sampling to the material bound to the hit object.
fn scene_sample(
    rng: &mut rand::rngs::ThreadRng,
    hit_record: &hittable::HitRecord,
    scene: &Scene,
    depth: u32,
) -> vec::Vec3 {
    let sample_start = time::Instant::now();
    let result = hit_record.renderable.sample(rng, hit_record, scene, depth);
    stats::add_sample_stat(stats::Stat::new(
        stats::SCENE_SAMPLE,
        sample_start.elapsed(),
    ));
    result
}

impl renderable::Renderable for Scene {
    fn hit(&self, ray: &ray::Ray, t_min: f32, t_max: f32) -> Option<hittable::HitRecord<'_>> {
        scene_hit(ray, self, t_min, t_max)
    }

    fn bounding_box(&self) -> super::bbox::BBox {
        if let Some(bvh) = &self.bvh {
            bvh.bounding_box().clone()
        } else {
            self.objects.bbox.clone()
        }
    }

    fn sample(
        &self,
        rng: &mut rand::rngs::ThreadRng,
        hit_record: &hittable::HitRecord<'_>,
        scene: &Scene,
        depth: u32,
    ) -> vec::Vec3 {
        scene_sample(rng, hit_record, scene, depth)
    }
}

impl renderable::Renderable for &Scene {
    fn hit(&self, ray: &ray::Ray, t_min: f32, t_max: f32) -> Option<hittable::HitRecord<'_>> {
        scene_hit(ray, self, t_min, t_max)
    }

    fn bounding_box(&self) -> super::bbox::BBox {
        if let Some(bvh) = &self.bvh {
            bvh.bounding_box().clone()
        } else {
            self.objects.bbox.clone()
        }
    }

    fn sample(
        &self,
        rng: &mut rand::rngs::ThreadRng,
        hit_record: &hittable::HitRecord<'_>,
        scene: &Scene,
        depth: u32,
    ) -> vec::Vec3 {
        scene_sample(rng, hit_record, scene, depth)
    }
}
