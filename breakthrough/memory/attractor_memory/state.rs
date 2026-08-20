// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use crate::MemoryError;

use super::*;

/// AttractorMemoryState: The current position in the attractor memory landscape.
///
/// Tracks where the cognitive system resides within the attractor network,
/// how close it is to equilibrium, and whether transitions between memory
/// basins are occurring.
#[derive(Debug, Clone, PartialEq)]
pub struct AttractorMemoryState {
    /// Current position in state space.
    pub current_state: Vec<f64>,
    /// Index of the currently active basin, if any.
    pub active_basin: Option<usize>,
    /// Stability of the current position in [0.0, 1.0].
    pub stability: f64,
    /// Distance threshold below which a transition is considered complete.
    pub transition_threshold: f64,
    /// Maximum number of transitions allowed per cycle.
    pub max_transitions: usize,
    /// Transition count in the current cycle.
    pub transition_count: usize,
}

impl AttractorMemoryState {
    /// Minimum dimensionality for state space.
    pub const MIN_DIM: usize = 1;
    /// Maximum dimensionality for state space.
    pub const MAX_DIM: usize = 1024;

    /// Creates a new AttractorMemoryState at the given position.
    ///
    /// # Errors
    ///
    /// Returns [`MemoryError::DimensionMismatch`] if the position is empty or
    /// exceeds the maximum dimension.
    pub fn new(position: Vec<f64>) -> Result<Self, MemoryError> {
        let dim = position.len();
        if dim == 0 || dim > Self::MAX_DIM {
            return Err(MemoryError::DimensionMismatch {
                expected: Self::MIN_DIM,
                actual: dim,
            });
        }
        Ok(Self {
            current_state: position,
            active_basin: None,
            stability: 0.0,
            transition_threshold: 0.01,
            max_transitions: 10,
            transition_count: 0,
        })
    }

    /// Attempts to transition the state toward a target basin strength.
    ///
    /// # Errors
    ///
    /// Returns [`MemoryError::ThresholdNotMet`] if the transition threshold
    /// cannot be reached.
    /// Returns [`MemoryError::CapacityExceeded`] if transition budget is exhausted.
    pub fn transition(&mut self, target_stability: f64) -> Result<(), MemoryError> {
        if self.transition_count >= self.max_transitions {
            return Err(MemoryError::CapacityExceeded {
                max: self.max_transitions,
                attempted: self.transition_count + 1,
            });
        }
        if !(0.0..=1.0).contains(&target_stability) {
            return Err(MemoryError::OutOfRange {
                field: "target_stability".into(),
                value: target_stability,
                min: 0.0,
                max: 1.0,
            });
        }
        self.stability = (self.stability + target_stability * 0.3).min(1.0);
        self.transition_count += 1;
        Ok(())
    }

    /// Stabilizes the state, increasing its robustness against perturbation.
    pub fn stabilize(&mut self, amount: f64) {
        self.stability = (self.stability + amount).min(1.0);
    }

    /// Returns whether the state is at equilibrium within its basin.
    pub fn is_at_equilibrium(&self) -> bool {
        self.stability >= (1.0 - self.transition_threshold)
    }

    /// Returns the potential energy of the current state.
    ///
    /// States far from any attractor have higher potential energy.
    pub fn potential_energy(&self) -> f64 {
        (1.0 - self.stability).clamp(0.0, 1.0)
    }

    /// Sets the active basin by index.
    pub fn set_basin(&mut self, basin_id: usize) {
        self.active_basin = Some(basin_id);
        self.transition_count = 0;
    }

    /// Returns whether the state has an active basin assigned.
    pub fn has_basin(&self) -> bool {
        self.active_basin.is_some()
    }

    /// Resets the transition budget for a new cognitive cycle.
    pub fn reset_transitions(&mut self) {
        self.transition_count = 0;
    }

    /// Validates the memory state.
    pub fn validate(&self) -> Result<(), MemoryError> {
        let dim = self.current_state.len();
        if dim == 0 || dim > Self::MAX_DIM {
            return Err(MemoryError::DimensionMismatch {
                expected: Self::MIN_DIM,
                actual: dim,
            });
        }
        if !(0.0..=1.0).contains(&self.stability) {
            return Err(MemoryError::OutOfRange {
                field: "stability".into(),
                value: self.stability,
                min: 0.0,
                max: 1.0,
            });
        }
        Ok(())
    }
}

