// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

//! Self Model: The internal representation of the system's own state.
//!
//! The self-model maintains three complementary representations:
//!
//! 1. **StateRepresentation** — A vector of current state values with
//!    confidence metadata, supporting L2 norm and normalization.
//! 2. **CapabilityModel** — Tracks registered capabilities, their capacity
//!    scores, and demonstrated proficiency.
//! 3. **BoundaryAwareness** — Maintains known limits and an uncertainty
//!    radius for safety-critical reasoning.
//!
//! # State Confidence
//!
//! Confidence is a scalar in [0.0, 1.0] reflecting how well the current
//! state vector matches observed reality. It decays under uncertainty and
//! recovers with corroborating observations.
//!
//! # Capability Profiling
//!
//! Each capability carries a capacity score (maximum potential) and a
//! proficiency score (current demonstrated ability). The overall score
//! is the capacity-weighted average proficiency.
//!
//! # Boundary Margins
//!
//! The uncertainty radius defines a safety margin around known limits.
//! Values inside `[radius, limit - radius]` are considered safe; values
//! outside trigger boundary-aware exception handling.
//!
//! # Examples
//!
//! ```
//! use breakthrough::cognition::self_model::{StateRepresentation, CapabilityModel, BoundaryAwareness, Capability};
//!
//! let mut state = StateRepresentation::new(vec![0.1, 0.5, 0.9], 0.8).expect("valid");
//! state.normalize();
//!
//! let mut model = CapabilityModel::new(4).expect("valid");
//! model.register_capability(1, 0.8).expect("registered");
//!
//! let bounds = BoundaryAwareness::new(vec![1.0, 1.0], 0.05).expect("valid");
//! assert!(bounds.is_within_bounds(0.5));
//! ```
//!
//! # Model Integration
//!
//! The three components should be updated together during each cognitive
//! cycle to maintain synchronization between state, capability, and boundary
//! awareness.
//!
//! [`CognitionError`]: super::CognitionError

pub mod state_representation;
pub mod capability_model;
pub mod boundary_awareness;

pub use super::CognitionError;
pub use state_representation::StateRepresentation;
pub use capability_model::{CapabilityModel, Capability};
pub use boundary_awareness::BoundaryAwareness;

/// Default confidence for newly initialized state representations.
pub const DEFAULT_CONFIDENCE: f64 = 0.8;
/// Hard cap for capability registration count.
pub const MAX_CAPABILITIES: usize = 10_000;
/// Default uncertainty radius for boundary awareness.
pub const DEFAULT_UNCERTAINTY_RADIUS: f64 = 0.05;

/// Creates a synchronized self-model bundle.
///
/// Initializes state, capabilities, and boundaries with coherent defaults.
///
/// # Errors
///
/// Returns [`CognitionError`] if any component fails initialization.
pub fn init_self_model(
    state_dim: usize,
    capability_count: usize,
) -> Result<(StateRepresentation, CapabilityModel, BoundaryAwareness), CognitionError> {
    let state_vector = vec![0.0; state_dim];
    let state = StateRepresentation::new(state_vector, DEFAULT_CONFIDENCE)?;
    let mut model = CapabilityModel::new(capability_count)?;
    let known_limits = vec![1.0; state_dim];
    let bounds = BoundaryAwareness::new(known_limits, DEFAULT_UNCERTAINTY_RADIUS)?;
    Ok((state, model, bounds))
}

/// Validates the self-model as a whole, ensuring cross-component consistency.
///
/// State dimensionality matches boundary dimensions, and capability
/// counts are within limits.
pub fn validate_self_model(
    state: &StateRepresentation,
    model: &CapabilityModel,
    bounds: &BoundaryAwareness,
) -> Result<(), CognitionError> {
    state.validate()?;
    model.validate()?;
    bounds.validate()?;
    if state.dimensionality() != bounds.dimension_count() {
        return Err(CognitionError::ConsistencyError(
            "state dimensionality does not match boundary dimensions".into(),
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_self_model_succeeds() {
        let (state, model, bounds) = init_self_model(3, 4).unwrap();
        assert_eq!(state.dimensionality(), 3);
        assert_eq!(bounds.dimension_count(), 3);
        assert_eq!(model.capability_count(), 0);
    }

    #[test]
    fn validate_detects_mismatch() {
        let state = StateRepresentation::new(vec![0.1, 0.2], DEFAULT_CONFIDENCE).unwrap();
        let model = CapabilityModel::new(1).unwrap();
        let bounds = BoundaryAwareness::new(vec![1.0, 1.0, 1.0], DEFAULT_UNCERTAINTY_RADIUS).unwrap();
        assert!(validate_self_model(&state, &model, &bounds).is_err());
    }
}