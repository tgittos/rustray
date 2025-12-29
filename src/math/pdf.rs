pub mod phase;
pub mod cosine;
pub mod uniform;

use rand::Rng;

use crate::math::vec;

/// Probability Density Function trait
pub trait PDF {
    fn value(&self, direction: vec::Vec3) -> f32;
    fn generate(&self, rng: &mut rand::rngs::ThreadRng) -> vec::Vec3;
}

/// Single PDF with an associated weight for mixture
pub struct PDFMix<'a> {
    pub pdf: Box<dyn PDF + Send + Sync + 'a>,
    pub weight: f32,
}

/// Mixture of multiple PDFs
pub struct MixturePDF<'a> {
    mixes: Vec<PDFMix<'a>>,
}

impl<'a> MixturePDF<'a> {
    pub fn new() -> Self {
        MixturePDF {
            mixes: Vec::new(),
        }
    }

    pub fn add(&mut self, pdf: Box<dyn PDF + Send + Sync + 'a>, weight: f32) {
        self.mixes.push(PDFMix { pdf, weight });
        self.balance_weights();
    }

    fn balance_weights(&mut self) {
        let total_weight: f32 = self.mixes.iter().map(|mix| mix.weight).sum();
        if total_weight > 0.0 {
            for mix in &mut self.mixes {
                mix.weight /= total_weight;
            }
        }
    }
}

impl PDF for MixturePDF<'_> {
    fn value(&self, direction: vec::Vec3) -> f32 {
        self.mixes
            .iter()
            .map(|mix| mix.weight * mix.pdf.value(direction))
            .sum()
    }

    fn generate(&self, rng: &mut rand::rngs::ThreadRng) -> vec::Vec3 {
        let r: f32 = rng.random::<f32>();
        let mut cumulative_weight = 0.0;
        for mix in &self.mixes {
            cumulative_weight += mix.weight;
            if r < cumulative_weight {
                return mix.pdf.generate(rng);
            }
        }

        self.mixes.last().unwrap().pdf.generate(rng)
    }
}
