// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

//! Attractor Memory: Memory encoded as stable attractor basins in state space.
//!
//! In dynamical systems theory, an attractor is a state toward which a system
//! tends to evolve. Attractor memory leverages this principle: memories are
//! represented as fixed points (or limit cycles) in a high-dimensional state
//! space. Perturbations that do not exceed the basin radius will return to the
//! attractor, providing inherent robustness to noise and partial damage.
//!
//! # Concepts
//!
//! - **Attractor Basin** — The region of state space from which the system
//!   will converge to a particular memory fixed point.
//! - **Stability** — The depth of the attractor potential well, governing
//!   resistance to perturbation and interference.
//! - **Occupancy** — The frequency and recency with which a basin has been
//!   visited, influencing its accessibility.
//!
//! # Examples
//!
//! ```
//! use breakthrough::memory::attractor_memory::{AttractorBasin, AttractorMemoryState};
//!
//! let mut basin = AttractorBasin::new(vec![0.5, 0.5], 0.3, 0.8).expect("valid");
//! basin.enroll(0.1);
//! assert!(basin.contains(&vec![0.5, 0.4]));
//!
//! let mut state = AttractorMemoryState::new(vec![0.1, 0.2]).expect("valid");
//! for _ in 0..10 {
//!     state.transition(1.0).unwrap();
//! }
//! assert!(state.is_at_equilibrium());
//! ```

use std::fmt;

use crate::MemoryError;

pub mod basin;
pub mod state;

pub use basin::AttractorBasin;
pub use state::AttractorMemoryState;

/// Default basin radius that defines the memory neighborhood.
pub const DEFAULT_BASIN_RADIUS: f64 = 0.3;
/// Maximum depth (potential well) for an attractor basin.
pub const MAX_BASIN_DEPTH: f64 = 1.0;
/// Minimum stability for a memory to be considered retrievable.
pub const MIN_RETRIEVAL_STABILITY: f64 = 0.4;
/// System-wide maximum number of attractor basins.
pub const MAX_BASINS: usize = 10_000;

/// Creates an attractor basin centered at the given coordinates.
///
/// # Errors
///
/// Returns [`MemoryError::DimensionMismatch`] if the center has zero dimensions.
/// Returns [`MemoryError::OutOfRange`] if depth is outside [0.0, 1.0].
pub fn create_basin(center: Vec<f64>, radius: f64, depth: f64) -> Result<AttractorBasin, MemoryError> {
    AttractorBasin::new(center, radius, depth)
}

/// Creates a new attractor memory state at the given position.
///
/// # Errors
///
/// Returns [`MemoryError::DimensionMismatch`] if the position vector is empty.
pub fn create_state(position: Vec<f64>) -> Result<AttractorMemoryState, MemoryError> {
    AttractorMemoryState::new(position)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basin_creation_succeeds() {
        let basin = create_basin(vec![0.5, 0.5], 0.3, 0.7).unwrap();
        assert!(basin.contains(&vec![0.5, 0.4]));
        assert!(basin.occupancy == 0);
    }

    #[test]
    fn basin_enroll_and_occupancy() {
        let mut basin = create_basin(vec![0.0, 0.0], 0.5, 0.6).unwrap();
        basin.enroll(0.5);
        assert_eq!(basin.occupancy, 1);
        basin.enroll(0.2);
        assert_eq!(basin.occupancy, 2);
    }

    #[test]
    fn basin_distance_to_center() {
        let basin = create_basin(vec![0.0, 0.0], 1.0, 0.5).unwrap();
        let d = basin.distance_to_center(&vec![3.0, 4.0]);
        assert!((d - 5.0).abs() < 1e-6);
    }

    #[test]
    fn state_transition_toward_equilibrium() {
        let mut state = create_state(vec![0.1, 0.1]).unwrap();
        for _ in 0..10 {
            state.transition(1.0).unwrap();
        }
        assert!(state.is_at_equilibrium());
    }

    #[test]
    fn state_potential_energy() {
        let state = create_state(vec![0.0, 0.0]).unwrap();
        assert_eq!(state.potential_energy(), 1.0);
    }

    #[test]
    fn default_constants_valid() {
        assert!(DEFAULT_BASIN_RADIUS > 0.0);
        assert!(MAX_BASIN_DEPTH > 0.0);
        assert!(MAX_BASINS > 0);
    }
}
