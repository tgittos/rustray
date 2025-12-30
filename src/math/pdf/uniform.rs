use crate::math::{pdf, vec};

pub struct UniformPDF {}

impl pdf::PDF for UniformPDF {
    fn value(&self, _direction: vec::Vec3) -> f32 {
        1.0 / (4.0 * std::f32::consts::PI)
    }

    fn generate(&self, rng: &mut rand::rngs::ThreadRng) -> vec::Vec3 {
        let z: f32 = 1.0 - 2.0 * rand::Rng::random::<f32>(rng);
        let r = (1.0 - z * z).sqrt();
        let phi = 2.0 * std::f32::consts::PI * rand::Rng::random::<f32>(rng);
        let x = r * phi.cos();
        let y = r * phi.sin();
        vec::Vec3::new(x, y, z)
    }
}
