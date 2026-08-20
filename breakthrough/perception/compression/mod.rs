// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

//! Compression: Reducing perceptual data to essential structure.
//! Removes redundancy while preserving semantic content.

pub mod lossless;
pub mod lossy;
pub mod latent;

pub use lossless::LosslessCompressor;
pub use lossy::LossyCompressor;
pub use latent::LatentCompressor;

pub use super::PerceptionResult;
pub use super::PerceptionError;
