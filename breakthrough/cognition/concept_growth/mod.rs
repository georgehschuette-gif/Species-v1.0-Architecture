// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

//! Concept Growth: The strengthening and elaboration of existing concepts.
//!
//! Growth mechanisms enrich concepts that are actively used, building
//! richer internal structure and tighter network integration.
//!
//! 1. **Reinforcement** — Increases concept strength each time the concept
//!    is employed, with diminishing returns as saturation approaches.
//! 2. **Elaboration** — Adds detail layers and branching structure,
//!    increasing conceptual complexity.
//! 3. **Integration** — Establishes connections to the broader knowledge
//!    network, amplifying retrieval robustness.
//!
//! # Diminishing Returns
//!
//! Reinforcement follows a saturation curve: the closer a concept is to
//! its maximum strength, the smaller each increment. This prevents any
//! single concept from dominating the cognitive landscape.
//!
//! # Examples
//!
//! ```
//! use breakthrough::cognition::concept_growth::{ConceptReinforcement, ConceptElaboration, ConceptIntegration};
//!
//! let mut reinforce = ConceptReinforcement::new(0.1, 1.0).expect("valid");
//! reinforce.apply(0.5).expect("reinforce");
//!
//! let mut elab = ConceptElaboration::new(2, 1.5).expect("valid");
//! elab.deepen().expect("deepen");
//!
//! let mut integrator = ConceptIntegration::new(0.5, 3).expect("valid");
//! integrator.add_connection().expect("connect");
//! ```
//!
//! # Capacity Management
//!
//! Growth is bounded by hard caps (100 elaboration layers, 10,000
//! connections) to preserve system memory and compute budgets.
//!
//! [`CognitionError`]: super::CognitionError

pub mod reinforcement;
pub mod elaboration;
pub mod integration;

pub use super::CognitionError;
pub use reinforcement::ConceptReinforcement;
pub use elaboration::ConceptElaboration;
pub use integration::ConceptIntegration;

/// Default reinforcement rate per utilization event.
pub const DEFAULT_REINFORCEMENT_RATE: f64 = 0.1;
/// Default branching factor for elaboration.
pub const DEFAULT_BRANCHING_FACTOR: f64 = 1.5;
/// Default integration link strength.
pub const DEFAULT_INTEGRATION_STRENGTH: f64 = 0.5;
/// Hard ceiling for elaboration depth.
pub const MAX_ELABORATION_DEPTH: usize = 100;
/// Hard ceiling for integration connections.
pub const MAX_INTEGRATION_CONNECTIONS: usize = 10_000;

/// Grows a concept by reinforcement, then elaboration, then integration.
///
/// Convenience pipeline returning the accumulated growth metrics.
///
/// # Errors
///
/// Propagates any [`CognitionError`] from the underlying mechanisms.
pub fn grow_concept(
    current_strength: f64,
    elaboration_layers: usize,
) -> Result<(f64, usize), CognitionError> {
    let mut reinforce = ConceptReinforcement::new(DEFAULT_REINFORCEMENT_RATE, 1.0)?;
    let strength = reinforce.apply(0.5)?;
    let mut elab = ConceptElaboration::new(elaboration_layers, DEFAULT_BRANCHING_FACTOR)?;
    elab.deepen_by(1)?;
    Ok((strength, elab.depth()))
}

/// Validates that a concept growth configuration is within system limits.
pub fn validate_growth_config(
    reinforcement_rate: f64,
    branching_factor: f64,
    integration_strength: f64,
) -> Result<(), CognitionError> {
    ConceptReinforcement::new(reinforcement_rate, 1.0)?.validate()?;
    ConceptElaboration::new(1, branching_factor)?.validate()?;
    ConceptIntegration::new(integration_strength, 0)?.validate()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn growth_pipeline_succeeds() {
        let (strength, depth) = grow_concept(0.5, 2).unwrap();
        assert!(strength > 0.0);
        assert!(depth > 2);
    }

    #[test]
    fn constants_are_sensible() {
        assert!(DEFAULT_REINFORCEMENT_RATE > 0.0);
        assert!(MAX_ELABORATION_DEPTH > 0);
    }
}