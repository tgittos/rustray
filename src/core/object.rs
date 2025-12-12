use std::sync::Arc;

use crate::core::{bbox, ray, scene};
use crate::geometry::instance::GeometryInstance;
use crate::materials::instance::MaterialInstance;
use crate::math::{interval, vec};
use crate::traits::hittable::Hittable;
use crate::traits::renderable::Renderable;
use crate::traits::sampleable::Sampleable;
use crate::traits::{hittable, sampleable};

/// A concrete implementation of the Renderable trait that combines a Hittable and a Sampleable.
/// This struct allows any object that implements both Hittable and Sampleable to be treated as a Renderable.
///
/// # Fields
/// [`hittable::Hittable`] hittable - The hittable component of the renderable.
/// [`sampleable::Sampleable`] sampleable - The sampleable component of the renderable.
pub struct RenderObject {
    /// Geometry that can be intersected.
    pub geometry_instance: GeometryInstance,
    pub material_instance: MaterialInstance,
}

impl RenderObject {
    /// Creates a new RenderObject from given Hittable and Sampleable objects.
    ///
    /// # Arguments
    /// * `hittable` - The hittable object.
    /// * `sampleable` - The sampleable object.
    pub fn new(
        hittable: Arc<dyn hittable::Hittable>,
        sampleable: Arc<dyn sampleable::Sampleable>,
    ) -> Self {
        let geometry_instance = GeometryInstance {
            ref_obj: hittable,
            transforms: Vec::new(),
        };
        let material_instance = MaterialInstance {
            ref_mat: sampleable,
            albedo: None,
        };
        RenderObject {
            geometry_instance,
            material_instance,
        }
    }
}

impl Renderable for RenderObject {
    fn hit(&self, ray: &ray::Ray, t_min: f32, t_max: f32) -> Option<hittable::HitRecord<'_>> {
        let maybe_hit = self.geometry_instance.hit(ray, t_min, t_max);
        if maybe_hit.is_none() {
            return None;
        }

        let hit = maybe_hit.unwrap();
        let hit_record = hittable::HitRecord {
            hit: hit,
            renderable: self,
        };

        Some(hit_record)
    }

    fn bounding_box(&self) -> bbox::BBox {
        self.geometry_instance.bounding_box()
    }

    fn sample(
        &self,
        rng: &mut rand::rngs::ThreadRng,
        hit_record: &hittable::HitRecord<'_>,
        scene: &scene::Scene,
        depth: u32,
    ) -> vec::Vec3 {
        self.material_instance.sample(rng, hit_record, scene, depth)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

/// A collection of renderable objects.
pub struct Renderables {
    pub objects: Vec<Box<dyn Renderable>>,

    pub bbox: bbox::BBox,
}

impl Renderables {
    /// Creates a new empty RenderableList.
    pub fn new() -> Self {
        Renderables {
            objects: Vec::new(),
            bbox: bbox::BBox::new(interval::empty(), interval::empty(), interval::empty()),
        }
    }

    /// Recomputes the aggregate bounding box from the stored objects.
    pub fn rebuild_bbox(&mut self) {
        self.bbox = self
            .objects
            .iter()
            .map(|obj| obj.bounding_box())
            .reduce(|acc, bbox| acc.union(&bbox))
            .unwrap_or_else(|| {
                bbox::BBox::new(interval::empty(), interval::empty(), interval::empty())
            });
    }

    /// Adds a hittable object to the list.
    pub fn add(&mut self, object: Box<dyn Renderable>) {
        let object_bbox = object.bounding_box();
        self.bbox = self.bbox.union(&object_bbox);
        self.objects.push(object);
    }

    pub fn len(&self) -> usize {
        self.objects.len()
    }
}
