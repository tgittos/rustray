//! Bounding Volume Hierarchy for accelerating renderable hit tests.
use crate::core::{bbox, ray};
use crate::traits::{hittable, renderable};

/// Internal BVH node representation.
pub enum BvhNode {
    Leaf {
        bounding_box: bbox::BBox,
        index: usize,
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
        objects: &[Box<dyn renderable::Renderable>],
        mut indices: Vec<usize>,
    ) -> Self {
        assert!(
            !indices.is_empty(),
            "BVH cannot be built without renderables"
        );

        if indices.len() == 1 {
            let index = indices.pop().unwrap();
            let bounding_box = objects[index].bounding_box();
            return BvhNode::Leaf {
                bounding_box,
                index,
            };
        }

        let bbox = indices
            .iter()
            .map(|&idx| objects[idx].bounding_box())
            .reduce(|acc, bbox| acc.union(&bbox))
            .unwrap();

        let axis = bbox.longest_axis();
        indices.sort_by(|a, b| BvhNode::box_compare(objects, *a, *b, axis));
        let mid = indices.len() / 2;
        let right_indices = indices.split_off(mid);
        let left_indices = indices;

        let left = Box::new(BvhNode::new(rng, objects, left_indices));
        let right = Box::new(BvhNode::new(rng, objects, right_indices));
        let bounding_box = left.bounding_box().union(right.bounding_box());

        BvhNode::Branch {
            bounding_box,
            left,
            right,
        }
    }

    fn hit<'a>(
        &'a self,
        objects: &'a [Box<dyn renderable::Renderable>],
        ray: &crate::core::ray::Ray,
        t_min: f32,
        t_max: f32,
    ) -> Option<hittable::HitRecord<'a>> {
        match self {
            BvhNode::Leaf { index, .. } => objects[*index].hit(ray, t_min, t_max),
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

                if let Some(left_hit) = left.hit(objects, ray, t_min, closest) {
                    closest = left_hit.hit.t;
                    hit_record = Some(left_hit);
                }

                if let Some(right_hit) = right.hit(objects, ray, t_min, closest) {
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
        objects: &[Box<dyn renderable::Renderable>],
        a: usize,
        b: usize,
        axis: usize,
    ) -> std::cmp::Ordering {
        let box_a = objects[a].bounding_box();
        let box_b = objects[b].bounding_box();

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
        objects: &[Box<dyn renderable::Renderable>],
    ) -> Self {
        let indices = (0..objects.len()).collect::<Vec<_>>();
        Bvh {
            root: BvhNode::new(rng, objects, indices),
        }
    }

    pub fn bounding_box(&self) -> &bbox::BBox {
        self.root.bounding_box()
    }

    pub fn hit<'a>(
        &'a self,
        objects: &'a [Box<dyn renderable::Renderable>],
        ray: &ray::Ray,
        t_min: f32,
        t_max: f32,
    ) -> Option<hittable::HitRecord<'a>> {
        self.root.hit(objects, ray, t_min, t_max)
    }
}
