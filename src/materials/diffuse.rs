//! Lambertian diffuse material that scatters light uniformly.
use crate::core::{ray, scene, vec};
use crate::traits::renderable::Renderable;
use crate::traits::{hittable, sampleable};

/// Diffuse surface with a constant albedo.
pub struct Diffuse {
    pub albedo: vec::Vec3,
}

impl Diffuse {
    /// Creates a new diffuse material with the given albedo.
    pub fn new(albedo: &vec::Vec3) -> Self {
        Diffuse { albedo: *albedo }
    }
}

/// Samples a diffuse bounce using cosine-weighted hemisphere sampling.
fn diffuse_sample(
    diffuse: &Diffuse,
    rng: &mut rand::rngs::ThreadRng,
    hit_record: &hittable::HitRecord,
    scene: &scene::Scene,
    depth: u32,
) -> vec::Vec3 {
    if depth == 0 {
        return vec::Vec3::new(0.0, 0.0, 0.0);
    }

    let hit = hit_record.hit;
    let target = hit.point + hit.normal + vec::random_in_unit_sphere(rng);

    // bounce ray and attenuate
    let new_ray = ray::Ray::new(&hit.point, &(target - hit.point), Some(hit.ray.time));
    if let Some(new_hit_record) = scene.hit(&new_ray, 0.001, f32::MAX) {
        let bounce = new_hit_record
            .renderable
            .sample(rng, &new_hit_record, scene, depth - 1);
        return diffuse.albedo * (0.5 * bounce);
    }

    // miss
    vec::Vec3::new(0.0, 0.0, 0.0)
}

impl sampleable::Sampleable for Diffuse {
    fn sample(
        &self,
        rng: &mut rand::rngs::ThreadRng,
        hit_record: &hittable::HitRecord,
        scene: &scene::Scene,
        depth: u32,
    ) -> vec::Vec3 {
        diffuse_sample(self, rng, hit_record, scene, depth)
    }
}

impl sampleable::Sampleable for &Diffuse {
    fn sample(
        &self,
        rng: &mut rand::rngs::ThreadRng,
        hit_record: &hittable::HitRecord,
        scene: &scene::Scene,
        depth: u32,
    ) -> vec::Vec3 {
        diffuse_sample(self, rng, hit_record, scene, depth)
    }
}
