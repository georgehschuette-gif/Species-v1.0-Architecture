// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

//! Diffusion: The spread of cognitive properties through the substrate.
//! Governs propagation, mixing, and gradient smoothing.

pub mod propagation;
pub mod mixing;
pub mod gradient;

pub use propagation::{PropagationField, DiffusionError};
pub use mixing::{CognitiveMixing, MixingError};
pub use gradient::{GradientSmoothing, GradientError};
