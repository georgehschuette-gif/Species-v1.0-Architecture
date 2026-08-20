// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

//! Entropy: The measure of disorder and uncertainty in the cognitive system.
//! Governs decay, noise, and the arrow of decreasing organization.

pub mod measure;
pub mod generation;
pub mod dissipation;

pub use measure::{EntropyMeasure, EntropyError};
pub use generation::{EntropyGeneration, EntropySource};
pub use dissipation::{EntropyDissipation, DissipationMechanism};
