use crate::math::{onb, pdf, vec};

pub struct CosinePDF {
    onb: onb::ONB,
}

impl CosinePDF {
    pub fn new(w: &vec::Vec3) -> Self {
        let onb = onb::ONB::build_from_w(w);
        Self { onb }
    }
}

impl pdf::PDF for CosinePDF {
    fn value(&self, direction: vec::Vec3) -> f32 {
        let cosine = vec::unit_vector(&direction).dot(&self.onb.w);
        if cosine <= 0.0 {
            0.0
        } else {
            cosine / std::f32::consts::PI
        }
    }

    fn generate(&self, rng: &mut rand::rngs::ThreadRng) -> vec::Vec3 {
        self.onb.local(&random_cosine_direction(rng))
    }
}

fn random_cosine_direction(rng: &mut rand::rngs::ThreadRng) -> vec::Vec3 {
    let r1: f32 = rand::Rng::random::<f32>(rng);
    let r2: f32 = rand::Rng::random::<f32>(rng);
    let z = (1.0 - r2).sqrt();

    let phi = 2.0 * std::f32::consts::PI * r1;
    let x = phi.cos() * r2.sqrt();
    let y = phi.sin() * r2.sqrt();

    vec::Vec3::new(x, y, z)
}
