use crate::math::vec;

pub trait Sampleable {
    fn sample_pixel(
        &self,
        rng: &mut rand::rngs::ThreadRng,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
    ) -> vec::Vec3;
}
