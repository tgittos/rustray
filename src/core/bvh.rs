//! Bounding Volume Hierarchy for accelerating renderable hit tests.
use crate::core::{bbox, ray};
use crate::traits::{hittable, renderable};

/// Internal BVH node representation.
pub enum BvhNode {
    Leaf {
        bounding_box: bbox::BBox,
        object: Box<dyn renderable::Renderable>,
    },
    Branch {
        bounding_box: bbox::BBox,
        left: Box<BvhNode>,
        right: Box<BvhNode>,
    },
}

impl BvhNode {
    fn new(
        rng: &mut rand::rngs::ThreadRng,
        mut renderables: Vec<Box<dyn renderable::Renderable>>,
    ) -> Self {
        assert!(
            !renderables.is_empty(),
            "BVH cannot be built without renderables"
        );

        if renderables.len() == 1 {
            let object = renderables.pop().unwrap();
            let bounding_box = object.bounding_box();
            return BvhNode::Leaf {
                bounding_box,
                object,
            };
        }

        let bbox = renderables
            .iter()
            .map(|obj| obj.bounding_box())
            .reduce(|acc, bbox| acc.union(&bbox))
            .unwrap();

        let axis = bbox.longest_axis();
        renderables.sort_by(|a, b| BvhNode::box_compare(a, b, axis));
        let mid = renderables.len() / 2;
        let right_renderables = renderables.split_off(mid);
        let left_renderables = renderables;

        let left = Box::new(BvhNode::new(rng, left_renderables));
        let right = Box::new(BvhNode::new(rng, right_renderables));
        let bounding_box = left.bounding_box().union(right.bounding_box());

        BvhNode::Branch {
            bounding_box,
            left,
            right,
        }
    }

    fn hit(
        &self,
        ray: &crate::core::ray::Ray,
        t_min: f32,
        t_max: f32,
    ) -> Option<hittable::HitRecord<'_>> {
        match self {
            BvhNode::Leaf { object, .. } => object.hit(ray, t_min, t_max),
            BvhNode::Branch {
                bounding_box,
                left,
                right,
            } => {
                if !bounding_box.hit(ray, t_min, t_max) {
                    return None;
                }

                let mut closest = t_max;
                let mut hit_record: Option<hittable::HitRecord> = None;

                if let Some(left_hit) = left.hit(ray, t_min, closest) {
                    closest = left_hit.hit.t;
                    hit_record = Some(left_hit);
                }

                if let Some(right_hit) = right.hit(ray, t_min, closest) {
                    hit_record = Some(right_hit);
                }

                hit_record
            }
        }
    }

    fn bounding_box(&self) -> &bbox::BBox {
        match self {
            BvhNode::Leaf { bounding_box, .. } => bounding_box,
            BvhNode::Branch { bounding_box, .. } => bounding_box,
        }
    }

    fn box_compare(
        a: &Box<dyn renderable::Renderable>,
        b: &Box<dyn renderable::Renderable>,
        axis: usize,
    ) -> std::cmp::Ordering {
        let box_a = a.bounding_box();
        let box_b = b.bounding_box();

        box_a
            .axis(axis)
            .min
            .partial_cmp(&box_b.axis(axis).min)
            .unwrap()
    }
}

/// BVH root wrapper that implements the `Renderable` trait.
pub struct Bvh {
    pub root: BvhNode,
}

impl Bvh {
    pub fn new(
        rng: &mut rand::rngs::ThreadRng,
        renderables: Vec<Box<dyn renderable::Renderable>>,
    ) -> Self {
        Bvh {
            root: BvhNode::new(rng, renderables),
        }
    }

    pub fn bounding_box(&self) -> &bbox::BBox {
        self.root.bounding_box()
    }

    pub fn hit(&self, ray: &ray::Ray, t_min: f32, t_max: f32) -> Option<hittable::HitRecord<'_>> {
        self.root.hit(ray, t_min, t_max)
    }
}
