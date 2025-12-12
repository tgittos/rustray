use serde::{Deserialize, Serialize};

use crate::math::{perlin, vec};
use crate::traits::texturable;

#[derive(Serialize)]
pub struct NoiseTexture {
    scale: f64,

    #[serde(skip)]
    perlin: perlin::PerlinGenerator,
}

impl Clone for NoiseTexture {
    fn clone(&self) -> Self {
        Self {
            scale: self.scale,
            perlin: perlin::PerlinGenerator::new(&mut rand::rng()),
        }
    }
}

impl NoiseTexture {
    pub fn new(rng: &mut rand::rngs::ThreadRng, scale: f64) -> Self {
        Self {
            scale,
            perlin: perlin::PerlinGenerator::new(rng),
        }
    }
}

impl<'de> Deserialize<'de> for NoiseTexture {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct NoiseTextureData {
            scale: f64,
        }

        let data = NoiseTextureData::deserialize(deserializer)?;
        Ok(Self {
            scale: data.scale,
            perlin: perlin::PerlinGenerator::new(&mut rand::rng()),
        })
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

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
