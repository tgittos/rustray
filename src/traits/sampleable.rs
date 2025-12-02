pub trait Sampleable {
    fn sample(&self, u: f32, v: f32) -> crate::Vec3;
}