// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

//! Concept Mutation: The transformation of concepts into new forms.
//!
//! Mutation enables concepts to adapt, evolve, and reconfigure in response
//! to environmental pressure or internal dynamics.
//!
//! 1. **Adaptation** — Adjusts a concept's internal parameters toward a
//!    target, bounded by plasticity.
//! 2. **Evolution** — Drives long-term structural changes under selection
//!    pressure, tracking fitness over epochs.
//! 3. **Reconfiguration** — Restructures the concept's internal architecture
//!    without changing external semantics.
//!
//! # Mutagenesis Process
//!
//! Mutation is typically applied during cognitive restructuring phases,
//! when the system detects that current concepts no longer fit the data.
//! The mutation rate controls how aggressively concepts transform.
//!
//! # Examples
//!
//! ```
//! use breakthrough::cognition::concept_mutation::{ConceptAdaptation, ConceptEvolution, ConceptReconfiguration};
//!
//! let mut adapt = ConceptAdaptation::new(0.15, 0.3).expect("valid");
//! adapt.adjust(0.5).expect("adjusted");
//!
//! let mut evolve = ConceptEvolution::new(0.2, 0.7).expect("valid");
//! evolve.evolve_epochs(10).expect("evolved");
//!
//! let mut reconf = ConceptReconfiguration::new(0.1, 0.2).expect("valid");
//! reconf.reconfigure_steps(5).expect(" reconfigured");
//! ```
//!
//! # Structural Stability
//!
//! After mutation, concepts are validated to ensure internal consistency.
//! Invalid states trigger retry with fallback parameters.
//!
//! [`CognitionError`]: super::CognitionError

pub mod adaptation;
pub mod evolution;
pub mod reconfiguration;

pub use super::CognitionError;
pub use adaptation::ConceptAdaptation;
pub use evolution::ConceptEvolution;
pub use reconfiguration::ConceptReconfiguration;

/// Default adaptation rate for incremental parameter adjustment.
pub const DEFAULT_ADAPTATION_RATE: f64 = 0.15;
/// Default structural plasticity for reconfiguration.
pub const DEFAULT_STRUCTURAL_PLASTICITY: f64 = 0.2;
/// Default evolution speed per epoch.
pub const DEFAULT_EVOLUTION_SPEED: f64 = 0.2;
/// Maximum structural complexity boundary for reconfiguration.
pub const MAX_STRUCTURAL_COMPLEXITY: f64 = 1.0;
/// Minimum structural complexity boundary for reconfiguration.
pub const MIN_STRUCTURAL_COMPLEXITY: f64 = 0.1;

/// Executes a full mutation cycle: adapt, evolve, then reconfigure.
///
/// Convenience wrapper for batch mutation of a concept's internal state.
///
/// # Errors
///
/// Returns [`CognitionError`] if any mutation step fails validation.
pub fn mutate_concept(
    target: f64,
    epochs: usize,
) -> Result<(ConceptAdaptation, ConceptEvolution, ConceptReconfiguration), CognitionError> {
    let mut adapt = ConceptAdaptation::new(DEFAULT_ADAPTATION_RATE, DEFAULT_STRUCTURAL_PLASTICITY)?;
    adapt.adjust(target)?;

    let mut evolve = ConceptEvolution::new(DEFAULT_EVOLUTION_SPEED, 0.7)?;
    evolve.evolve_epochs(epochs)?;

    let mut reconf = ConceptReconfiguration::new(DEFAULT_STRUCTURAL_PLASTICITY, DEFAULT_STRUCTURAL_PLASTICITY)?;
    reconf.reconfigure_steps(1)?;

    Ok((adapt, evolve, reconf))
}

/// Validates that mutation parameters are within safe operating bounds.
pub fn validate_mutation_params(
    adaptation_rate: f64,
    evolution_speed: f64,
    reconfiguration_rate: f64,
) -> Result<(), CognitionError> {
    ConceptAdaptation::new(adaptation_rate, DEFAULT_STRUCTURAL_PLASTICITY)?.validate()?;
    ConceptEvolution::new(evolution_speed, 0.5)?.validate()?;
    ConceptReconfiguration::new(reconfiguration_rate, DEFAULT_STRUCTURAL_PLASTICITY)?.validate()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mutation_cycle_succeeds() {
        let result = mutate_concept(0.6, 5);
        assert!(result.is_ok());
        let (adapt, evolve, reconf) = result.unwrap();
        assert!(adapt.current_deviation() >= 0.0);
        assert!(evolve.fitness() >= 0.0);
        assert!(reconf.complexity() >= MIN_STRUCTURAL_COMPLEXITY);
        assert!(reconf.complexity() <= MAX_STRUCTURAL_COMPLEXITY);
    }

    #[test]
    fn validation_catches_bad_params() {
        assert!(validate_mutation_params(-0.1, 0.5, 0.2).is_err());
        assert!(validate_mutation_params(0.1, 1.5, 0.2).is_err());
    }
}