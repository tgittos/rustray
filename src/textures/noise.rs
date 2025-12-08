use crate::core::vec;
use crate::traits::texturable;
use crate::utils::perlin;

pub struct NoiseTexture {
    scale: f64,
    perlin: perlin::PerlinGenerator,
}

impl NoiseTexture {
    pub fn new(rng: &mut rand::rngs::ThreadRng, scale: f64) -> Self {
        Self {
            scale,
            perlin: perlin::PerlinGenerator::new(rng),
        }
    }
}

impl texturable::Texturable for NoiseTexture {
    fn sample(&self, hit_record: &crate::traits::hittable::Hit) -> vec::Vec3 {
        let scaled_point = hit_record.point * self.scale;
        // Marble-like effect using turbulent Perlin noise; stays positive for gamma correction.
        let marble = (scaled_point.z + 10.0 * self.perlin.turbulence(scaled_point, 7)).sin();
        let noise_value = 0.5 * (1.0 + marble);

        vec::Point3::new(1.0, 1.0, 1.0) * noise_value
    }
}
