//! Traits module containing definitions for Hittable, Renderable, and Sampleable.
//!
//! # Hittable
//! The [hittable::Hittable] trait defines objects that can be intersected by rays. It
//! includes a method to determine if a ray hits the object within a specified
//! range.
//!
//! # Sampleable
//! The [sampleable::Sampleable] trait defines objects that can provide color contributions
//! based on ray intersections. It includes a method to sample color at hit points.
//!
//! # Renderable
//! The [renderable::Renderable] trait combines `Hittable` and `Sampleable` traits, allowing
//! objects to be both intersected by rays and provide color contributions.
//!
//! # Texturable
//! The [texturable::Texturable] trait defines objects that can provide texture color values
//! based on texture coordinates and points in space.

pub mod hittable;
pub mod renderable;
pub mod sampleable;
pub mod texturable;
