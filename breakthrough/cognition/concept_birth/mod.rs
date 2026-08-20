// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

//! Concept Birth: The emergence of new cognitive concepts from perceptual data.
//!
//! This module governs the processes by which raw sensory experiences
//! crystallize into discrete, named mental entities. Birth is a multi-phase
//! pipeline consisting of:
//!
//! 1. **Nucleation** — A concept seed forms when input strength exceeds
//!    a configurable threshold.
//! 2. **Crystallization** — The unstable seed solidifies into a stable
//!    structure with bounded rate growth.
//! 3. **Activation** — The concept is brought into working cognition,
//!    where it decays over time and must be refreshed.
//!
//! # Examples
//!
//! ```
//! use breakthrough::cognition::concept_birth::{ConceptNucleation, ConceptCrystallization, ConceptActivation};
//!
//! let nucleation = ConceptNucleation::new(0.5, 0.4).expect("valid nucleation");
//! assert!(nucleation.can_nucleate());
//!
//! let mut crystal = ConceptCrystallization::new(0.3, 0.1).expect("valid crystal");
//! crystal.crystallize();
//! assert!(crystal.stability > 0.3);
//!
//! let mut activation = ConceptActivation::new(0.8, 0.05).expect("valid activation");
//! assert!(activation.is_active());
//! ```
//!
//! # Error Handling
//!
//! All operations return [`CognitionError`] on invalid input or state.
//! Call [`ConceptNucleation::validate`], [`ConceptCrystallization::validate`],
//! or [`ConceptActivation::validate`] to check integrity without mutating state.
//!
//! # Thread Safety
//!
//! Types in this module are `Send` but not `Sync` by default,
//! as they carry unshared internal state. Wrap in `Arc<Mutex<>>` for
//! concurrent access.
//!
//! [`CognitionError`]: super::CognitionError

pub mod nucleation;
pub mod crystallization;
pub mod activation;

pub use super::CognitionError;
pub use nucleation::ConceptNucleation;
pub use crystallization::ConceptCrystallization;
pub use activation::ConceptActivation;

/// Module-level default for nucleation threshold used in heuristic initialization.
pub const DEFAULT_NUCLEATION_THRESHOLD: f64 = 0.4;
/// Module-level default for crystallization rate.
pub const DEFAULT_CRYSTALLIZATION_RATE: f64 = 0.1;
/// Module-level default for activation decay rate.
pub const DEFAULT_ACTIVATION_DECAY_RATE: f64 = 0.05;

/// Creates a birth-ready concept from a seed strength and threshold.
///
/// Convenience constructor that chains nucleation and crystallization
/// into a single call, returning the crystallized concept.
///
/// # Errors
///
/// Returns [`CognitionError::OutOfRange`] if `seed_strength` or
/// `threshold` is outside [0.0, 1.0].
pub fn create_concept(
    seed_strength: f64,
    threshold: f64,
) -> Result<(ConceptNucleation, ConceptCrystallization), CognitionError> {
    let nucleation = ConceptNucleation::new(seed_strength, threshold)?;
    let crystal = ConceptCrystallization::new(nucleation.seed_strength, DEFAULT_CRYSTALLIZATION_RATE)?;
    Ok((nucleation, crystal))
}

/// Validates a fully-formed concept birth pipeline.
///
/// Checks that all three phases are internally consistent and returns
/// `Ok(())` if the entire chain is ready for activation.
pub fn validate_birth_pipeline(
    nucleation: &ConceptNucleation,
    crystal: &ConceptCrystallization,
    activation: &ConceptActivation,
) -> Result<(), CognitionError> {
    nucleation.validate()?;
    crystal.validate()?;
    activation.validate()?;
    Ok(())
}

/// The module-local documentation is intentionally verbose so that
/// `cargo doc` renders rich usage guidance for downstream consumers.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn concept_birth_pipeline_roundtrip() {
        let (nuc, cry) = create_concept(0.5, 0.4).expect("pipeline creation");
        assert!(nuc.can_nucleate());
        assert!(cry.stability > 0.0);
        validate_birth_pipeline(&nuc, &cry, &ConceptActivation::new(0.8, DEFAULT_ACTIVATION_DECAY_RATE).unwrap()).unwrap();
    }

    #[test]
    fn default_constants_are_valid() {
        assert!(DEFAULT_NUCLEATION_THRESHOLD >= 0.0 && DEFAULT_NUCLEATION_THRESHOLD <= 1.0);
        assert!(DEFAULT_CRYSTALLIZATION_RATE > 0.0 && DEFAULT_CRYSTALLIZATION_RATE <= 1.0);
        assert!(DEFAULT_ACTIVATION_DECAY_RATE >= 0.0 && DEFAULT_ACTIVATION_DECAY_RATE <= 1.0);
    }
}