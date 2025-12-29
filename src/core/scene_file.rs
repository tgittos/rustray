use std::collections::HashMap;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::core::{camera, object, render, scene, volume, world};
use crate::geometry::{
    instance::GeometryInstance,
    primitives::{cube, quad, sphere},
    transform,
};
use crate::materials::{
    dielectric, diffuse_light, instance::MaterialInstance, lambertian, metallic,
};
use crate::math::vec;
use crate::textures::{checker, color, noise, uv};
use crate::traits::{hittable, sampleable, texturable};

#[derive(Serialize, Deserialize)]
pub struct SceneFile {
    pub width: u32,
    pub samples: u32,
    pub depth: u32,
    pub camera: camera::Camera,
    pub geometries: Vec<GeometryEntry>,
    pub materials: Vec<MaterialEntry>,
    pub objects: Vec<ObjectInstance>,
    #[serde(default)]
    pub volumes: Vec<VolumeInstance>,
}

#[derive(Serialize, Deserialize)]
pub struct GeometryEntry {
    pub id: usize,
    #[serde(flatten)]
    pub geometry: GeometryTemplate,
}

#[derive(Serialize, Deserialize)]
pub struct MaterialEntry {
    pub id: usize,
    #[serde(flatten)]
    pub material: MaterialTemplate,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ObjectInstance {
    pub geometry: usize,
    pub material: usize,
    #[serde(default)]
    pub transforms: Vec<transform::Transform>,
    pub albedo: Option<vec::Vec3>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct VolumeInstance {
    pub boundary_geometry: usize,
    pub phase_function: usize,
    pub density: f32,
    #[serde(default)]
    pub boundary_transforms: Vec<transform::Transform>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(tag = "hittable", content = "data")]
pub enum GeometryTemplate {
    Sphere(sphere::Sphere),
    Quad(quad::Quad),
    Cube(cube::Cube),
    World(world::World),
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(tag = "sampleable", content = "data")]
pub enum MaterialTemplate {
    Lambertian { texture: TextureTemplate },
    Metallic(metallic::Metallic),
    Dielectric(dielectric::Dielectric),
    DiffuseLight { texture: TextureTemplate },
    Isotropic { texture: TextureTemplate },
    World(world::World),
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(tag = "texturable", content = "data")]
pub enum TextureTemplate {
    Color(color::ColorTexture),
    Checker(checker::CheckerTexture),
    Noise(noise::NoiseTexture),
    Uv(uv::UvTexture),
}

#[derive(Debug)]
pub enum SceneFileError {
    Io(std::io::Error),
    TomlDe(toml::de::Error),
    TomlSer(toml::ser::Error),
    UnsupportedRenderable(String),
    UnsupportedGeometry(String),
    UnsupportedMaterial(String),
    UnsupportedTexture(String),
    MissingGeometry(usize),
    MissingMaterial(usize),
}

impl std::fmt::Display for SceneFileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SceneFileError::Io(err) => write!(f, "{}", err),
            SceneFileError::TomlDe(err) => write!(f, "{}", err),
            SceneFileError::TomlSer(err) => write!(f, "{}", err),
            SceneFileError::UnsupportedRenderable(kind) => {
                write!(f, "unsupported renderable type: {}", kind)
            }
            SceneFileError::UnsupportedGeometry(kind) => {
                write!(f, "unsupported geometry type: {}", kind)
            }
            SceneFileError::UnsupportedMaterial(kind) => {
                write!(f, "unsupported material type: {}", kind)
            }
            SceneFileError::UnsupportedTexture(kind) => {
                write!(f, "unsupported texture type: {}", kind)
            }
            SceneFileError::MissingGeometry(id) => write!(f, "missing geometry id {}", id),
            SceneFileError::MissingMaterial(id) => write!(f, "missing material id {}", id),
        }
    }
}

impl std::error::Error for SceneFileError {}

impl From<std::io::Error> for SceneFileError {
    fn from(value: std::io::Error) -> Self {
        SceneFileError::Io(value)
    }
}

impl From<toml::de::Error> for SceneFileError {
    fn from(value: toml::de::Error) -> Self {
        SceneFileError::TomlDe(value)
    }
}

impl From<toml::ser::Error> for SceneFileError {
    fn from(value: toml::ser::Error) -> Self {
        SceneFileError::TomlSer(value)
    }
}

impl SceneFile {
    pub fn from_render(render: &render::Render) -> Result<Self, SceneFileError> {
        let mut builder = RegistryBuilder::default();
        let mut objects: Vec<ObjectInstance> = Vec::new();
        let mut volumes: Vec<VolumeInstance> = Vec::new();

        for renderable in render.scene.renderables.objects.iter() {
            if let Some(render_object) = renderable.as_any().downcast_ref::<object::RenderObject>()
            {
                let geometry_id =
                    builder.register_geometry(&render_object.geometry_instance.ref_obj)?;
                let material_id =
                    builder.register_material(&render_object.material_instance.ref_mat)?;

                objects.push(ObjectInstance {
                    geometry: geometry_id,
                    material: material_id,
                    transforms: render_object.geometry_instance.transforms.clone(),
                    albedo: render_object.material_instance.albedo,
                });
                continue;
            }

            if let Some(render_volume) = renderable.as_any().downcast_ref::<volume::RenderVolume>()
            {
                let boundary = render_volume
                    .boundary
                    .as_any()
                    .downcast_ref::<GeometryInstance>()
                    .ok_or_else(|| {
                        SceneFileError::UnsupportedRenderable(
                            "RenderVolume boundary must be GeometryInstance".to_string(),
                        )
                    })?;

                let geometry_id = builder.register_geometry(&boundary.ref_obj)?;
                let phase_function_id = builder.register_material(&render_volume.phase_function)?;

                volumes.push(VolumeInstance {
                    boundary_geometry: geometry_id,
                    phase_function: phase_function_id,
                    density: render_volume.density,
                    boundary_transforms: boundary.transforms.clone(),
                });
                continue;
            }

            return Err(SceneFileError::UnsupportedRenderable(
                "unknown renderable type".to_string(),
            ));
        }

        Ok(SceneFile {
            width: render.width,
            samples: render.samples,
            depth: render.depth,
            camera: render.camera.clone(),
            geometries: builder.geometries,
            materials: builder.materials,
            objects,
            volumes,
        })
    }

    pub fn into_render(
        self,
        rng: &mut rand::rngs::ThreadRng,
    ) -> Result<render::Render, SceneFileError> {
        let geometries: Vec<_> = self
            .geometries
            .iter()
            .map(|entry| entry.geometry.to_hittable())
            .collect();
        let materials: Vec<_> = self
            .materials
            .iter()
            .map(|entry| entry.material.to_sampleable())
            .collect::<Result<_, _>>()?;

        let mut scene = scene::Scene::new();
        for object in self.objects.into_iter() {
            let Some(geometry) = geometries.get(object.geometry) else {
                return Err(SceneFileError::MissingGeometry(object.geometry));
            };
            let Some(material) = materials.get(object.material) else {
                return Err(SceneFileError::MissingMaterial(object.material));
            };

            let albedo = object.albedo;
            let transforms = object.transforms;
            let geometry_instance = GeometryInstance {
                ref_obj: geometry.clone(),
                transforms: transforms.clone(),
            };
            let material_instance = MaterialInstance {
                ref_mat: material.clone(),
                albedo,
            };

            let render_object = object::RenderObject {
                geometry_instance,
                material_instance,
            };
            let is_emissive = render_object
                .material_instance
                .ref_mat
                .as_any()
                .downcast_ref::<diffuse_light::DiffuseLight>()
                .is_some();

            scene.add_object(Box::new(render_object));

            if is_emissive {
                let light_geometry = GeometryInstance {
                    ref_obj: geometry.clone(),
                    transforms,
                };
                let light_material = MaterialInstance {
                    ref_mat: material.clone(),
                    albedo,
                };
                scene.add_light(Box::new(object::RenderObject {
                    geometry_instance: light_geometry,
                    material_instance: light_material,
                }));
            }
        }
        for volume in self.volumes.into_iter() {
            let Some(geometry) = geometries.get(volume.boundary_geometry) else {
                return Err(SceneFileError::MissingGeometry(volume.boundary_geometry));
            };
            let Some(phase_function) = materials.get(volume.phase_function) else {
                return Err(SceneFileError::MissingMaterial(volume.phase_function));
            };

            let boundary = GeometryInstance {
                ref_obj: geometry.clone(),
                transforms: volume.boundary_transforms,
            };

            scene.add_object(Box::new(volume::RenderVolume::new(
                Box::new(boundary),
                volume.density,
                phase_function.clone(),
            )));
        }
        scene.build_bvh(rng);

        Ok(render::Render {
            width: self.width,
            samples: self.samples,
            depth: self.depth,
            camera: self.camera,
            scene,
        })
    }
}

pub fn load_render(
    rng: &mut rand::rngs::ThreadRng,
    path: &Path,
) -> Result<render::Render, SceneFileError> {
    let content = std::fs::read_to_string(path)?;
    let scene_file: SceneFile = toml::from_str(&content)?;
    scene_file.into_render(rng)
}

pub fn save_render(render: &render::Render, path: &Path) -> Result<(), SceneFileError> {
    let file = SceneFile::from_render(render)?;
    let content = toml::to_string(&file)?;
    std::fs::write(path, content)?;
    Ok(())
}

#[derive(Default)]
struct RegistryBuilder {
    geometry_ids: HashMap<usize, usize>,
    material_ids: HashMap<usize, usize>,
    geometries: Vec<GeometryEntry>,
    materials: Vec<MaterialEntry>,
}

impl RegistryBuilder {
    fn register_geometry(
        &mut self,
        geometry: &std::sync::Arc<dyn hittable::Hittable + Send + Sync>,
    ) -> Result<usize, SceneFileError> {
        let key = arc_key(geometry);
        if let Some(existing) = self.geometry_ids.get(&key) {
            return Ok(*existing);
        }

        let entry = GeometryEntry {
            id: self.geometries.len(),
            geometry: GeometryTemplate::from_hittable(geometry)?,
        };
        self.geometry_ids.insert(key, entry.id);
        self.geometries.push(entry);
        Ok(self.geometries.len() - 1)
    }

    fn register_material(
        &mut self,
        material: &std::sync::Arc<dyn sampleable::Sampleable + Send + Sync>,
    ) -> Result<usize, SceneFileError> {
        let key = arc_key(material);
        if let Some(existing) = self.material_ids.get(&key) {
            return Ok(*existing);
        }

        let entry = MaterialEntry {
            id: self.materials.len(),
            material: MaterialTemplate::from_sampleable(material)?,
        };
        self.material_ids.insert(key, entry.id);
        self.materials.push(entry);
        Ok(self.materials.len() - 1)
    }
}

impl GeometryTemplate {
    fn from_hittable(
        hittable: &std::sync::Arc<dyn hittable::Hittable + Send + Sync>,
    ) -> Result<Self, SceneFileError> {
        if let Some(sphere) = hittable.as_any().downcast_ref::<sphere::Sphere>() {
            return Ok(GeometryTemplate::Sphere(sphere.clone()));
        }
        if let Some(quad) = hittable.as_any().downcast_ref::<quad::Quad>() {
            return Ok(GeometryTemplate::Quad(quad.clone()));
        }
        if let Some(cube) = hittable.as_any().downcast_ref::<cube::Cube>() {
            return Ok(GeometryTemplate::Cube(cube.clone()));
        }
        if let Some(world) = hittable.as_any().downcast_ref::<world::World>() {
            return Ok(GeometryTemplate::World(*world));
        }

        Err(SceneFileError::UnsupportedGeometry(
            "unknown hittable".to_string(),
        ))
    }

    fn to_hittable(&self) -> std::sync::Arc<dyn hittable::Hittable + Send + Sync> {
        match self {
            GeometryTemplate::Sphere(sphere) => std::sync::Arc::new(sphere.clone())
                as std::sync::Arc<dyn hittable::Hittable + Send + Sync>,
            GeometryTemplate::Quad(quad) => std::sync::Arc::new(quad.clone())
                as std::sync::Arc<dyn hittable::Hittable + Send + Sync>,
            GeometryTemplate::Cube(cube) => std::sync::Arc::new(cube.clone())
                as std::sync::Arc<dyn hittable::Hittable + Send + Sync>,
            GeometryTemplate::World(world) => {
                std::sync::Arc::new(*world) as std::sync::Arc<dyn hittable::Hittable + Send + Sync>
            }
        }
    }
}

impl MaterialTemplate {
    fn from_sampleable(
        material: &std::sync::Arc<dyn sampleable::Sampleable + Send + Sync>,
    ) -> Result<Self, SceneFileError> {
        if let Some(lambert) = material.as_any().downcast_ref::<lambertian::Lambertian>() {
            return Ok(MaterialTemplate::Lambertian {
                texture: TextureTemplate::from_texturable(lambert.texture.as_ref())?,
            });
        }
        if let Some(isotropic) = material.as_any().downcast_ref::<volume::Isotropic>() {
            return Ok(MaterialTemplate::Isotropic {
                texture: TextureTemplate::from_texturable(isotropic.texture.as_ref())?,
            });
        }
        if let Some(metal) = material.as_any().downcast_ref::<metallic::Metallic>() {
            return Ok(MaterialTemplate::Metallic(metal.clone()));
        }
        if let Some(dielectric) = material.as_any().downcast_ref::<dielectric::Dielectric>() {
            return Ok(MaterialTemplate::Dielectric(dielectric.clone()));
        }
        if let Some(diffuse_light) = material
            .as_any()
            .downcast_ref::<diffuse_light::DiffuseLight>()
        {
            return Ok(MaterialTemplate::DiffuseLight {
                texture: TextureTemplate::from_texturable(diffuse_light.texture.as_ref())?,
            });
        }
        if let Some(world) = material.as_any().downcast_ref::<world::World>() {
            return Ok(MaterialTemplate::World(*world));
        }

        Err(SceneFileError::UnsupportedMaterial(
            "unknown material".to_string(),
        ))
    }

    fn to_sampleable(
        &self,
    ) -> Result<std::sync::Arc<dyn sampleable::Sampleable + Send + Sync>, SceneFileError> {
        let material: std::sync::Arc<dyn sampleable::Sampleable + Send + Sync> = match self {
            MaterialTemplate::Lambertian { texture } => {
                std::sync::Arc::new(lambertian::Lambertian::new(texture.to_texturable()?))
            }
            MaterialTemplate::Isotropic { texture } => {
                std::sync::Arc::new(volume::Isotropic::new(texture.to_texturable()?))
            }
            MaterialTemplate::Metallic(metal) => std::sync::Arc::new(metal.clone())
                as std::sync::Arc<dyn sampleable::Sampleable + Send + Sync>,
            MaterialTemplate::Dielectric(dielectric) => std::sync::Arc::new(dielectric.clone())
                as std::sync::Arc<dyn sampleable::Sampleable + Send + Sync>,
            MaterialTemplate::DiffuseLight { texture } => {
                std::sync::Arc::new(diffuse_light::DiffuseLight::new(texture.to_texturable()?))
            }
            MaterialTemplate::World(world) => std::sync::Arc::new(*world)
                as std::sync::Arc<dyn sampleable::Sampleable + Send + Sync>,
        };

        Ok(material)
    }
}

impl TextureTemplate {
    fn from_texturable(texture: &dyn texturable::Texturable) -> Result<Self, SceneFileError> {
        if let Some(color) = texture.as_any().downcast_ref::<color::ColorTexture>() {
            return Ok(TextureTemplate::Color(color.clone()));
        }
        if let Some(checker) = texture.as_any().downcast_ref::<checker::CheckerTexture>() {
            return Ok(TextureTemplate::Checker(checker.clone()));
        }
        if let Some(noise) = texture.as_any().downcast_ref::<noise::NoiseTexture>() {
            return Ok(TextureTemplate::Noise(noise.clone()));
        }
        if let Some(uv) = texture.as_any().downcast_ref::<uv::UvTexture>() {
            return Ok(TextureTemplate::Uv(uv.clone()));
        }

        Err(SceneFileError::UnsupportedTexture(
            "unknown texture".to_string(),
        ))
    }

    fn to_texturable(
        &self,
    ) -> Result<Box<dyn texturable::Texturable + Send + Sync>, SceneFileError> {
        let texture: Box<dyn texturable::Texturable + Send + Sync> = match self {
            TextureTemplate::Color(color) => Box::new(color.clone()),
            TextureTemplate::Checker(checker) => Box::new(checker.clone()),
            TextureTemplate::Noise(noise) => Box::new(noise.clone()),
            TextureTemplate::Uv(uv) => Box::new(uv.clone()),
        };

        Ok(texture)
    }
}

fn arc_key<T: ?Sized>(arc: &std::sync::Arc<T>) -> usize {
    let ptr = std::sync::Arc::as_ptr(arc);
    ptr as *const () as usize
}
