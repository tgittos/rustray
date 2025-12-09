//! Glue trait combining geometry (hittable) and material sampling.
use serde::{Deserialize, Serialize};

use crate::core::{bbox, interval, ray, scene, vec};
use crate::traits::hittable;
use crate::traits::sampleable;

/// Trait for objects that can be rendered in the scene.
#[typetag::serde(tag = "renderable")]
pub trait Renderable {
    /// Determines if a ray hits the renderable object within the given t range.
    /// Returns [`hittable::HitRecord`] Some(HitRecord) if there is a hit, otherwise None.
    ///
    /// # Arguments
    /// * [`ray::Ray`] `ray` - The ray to test for intersection.
    /// * `t_min` - The minimum t value for valid intersections.
    /// * `t_max` - The maximum t value for valid intersections.
    ///
    /// # Returns
    /// An Option containing a [`hittable::HitRecord`] HitRecord if the ray hits the object, otherwise None.
    fn hit(&self, ray: &ray::Ray, t_min: f32, t_max: f32) -> Option<hittable::HitRecord<'_>>;

    /// Returns the bounding box of the renderable object.
    fn bounding_box(&self) -> bbox::BBox;

    /// Samples the color contribution at the hit point.
    fn sample(
        &self,
        rng: &mut rand::rngs::ThreadRng,
        hit_record: &hittable::HitRecord,
        scene: &scene::Scene,
        depth: u32,
    ) -> vec::Vec3;
}

/// A concrete implementation of the Renderable trait that combines a Hittable and a Sampleable.
/// This struct allows any object that implements both Hittable and Sampleable to be treated as a Renderable.
///
/// # Fields
/// [`hittable::Hittable`] hittable - The hittable component of the renderable.
/// [`sampleable::Sampleable`] sampleable - The sampleable component of the renderable.
#[derive(Serialize, Deserialize)]
pub struct RenderableImpl {
    /// Geometry that can be intersected.
    pub hittable: Box<dyn hittable::Hittable>,
    /// Material that determines color contribution.
    pub sampleable: Box<dyn sampleable::Sampleable>,
}

#[typetag::serde]
impl Renderable for RenderableImpl {
    fn hit(&self, ray: &ray::Ray, t_min: f32, t_max: f32) -> Option<hittable::HitRecord<'_>> {
        let maybe_hit = self.hittable.hit(ray, t_min, t_max);
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
        self.hittable.bounding_box()
    }

    fn sample(
        &self,
        rng: &mut rand::rngs::ThreadRng,
        hit_record: &hittable::HitRecord<'_>,
        scene: &scene::Scene,
        depth: u32,
    ) -> vec::Vec3 {
        self.sampleable.sample(rng, hit_record, scene, depth)
    }
}

/// A collection of renderable objects.
#[derive(Serialize, Deserialize)]
pub struct RenderableList {
    pub objects: Vec<Box<dyn Renderable>>,

    #[serde(skip)]
    pub bbox: bbox::BBox,
}

impl RenderableList {
    /// Creates a new empty RenderableList.
    pub fn new() -> Self {
        RenderableList {
            objects: Vec::new(),
            bbox: bbox::BBox::new(interval::empty(), interval::empty(), interval::empty()),
        }
    }

    /// Adds a hittable object to the list.
    pub fn add(&mut self, object: Box<dyn Renderable>) {
        let object_bbox = object.bounding_box();
        self.bbox = self.bbox.union(&object_bbox);
        self.objects.push(object);
    }
}

/// Helper function to create a RenderableImpl from given Hittable and Sampleable objects.
///
/// # Arguments
/// [`hittable::Hittable`] The hittable object.
/// [`sampleable::Sampleable`] The sampleable object.
pub fn create_renderable(
    hittable: Box<dyn hittable::Hittable>,
    sampleable: Box<dyn sampleable::Sampleable>,
) -> RenderableImpl {
    RenderableImpl {
        hittable,
        sampleable,
    }
}
