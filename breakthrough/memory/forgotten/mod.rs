// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

//! Forgotten: Decay, suppression, and dissolution of memory.
//!
//! Forgetting is not merely passive decay. This module models multiple
//! forgetting mechanisms: exponential decay, accelerated loss for unused
//! memories, active suppression by inhibitory forces, and dissolution below
//! the threshold of retrievability.
//!
//! # Components
//!
//! - **MemoryDecay** — Passive and accelerated forgetting curves with
//!   configurable decay types.
//! - **MemorySuppression** — Active inhibitory mechanisms that reduce memory
//!   strength below baseline, with potential for recovery.
//!
//! # Decay Types
//!
//! - **Exponential** — Standard Ebbinghaus-like forgetting curve.
//! - **Power Law** — Rapid initial loss that flattens over time.
//! - **Step** — Abrupt loss after a retention interval.
//!
//! # Suppression
//!
//! Suppression is modeled as an external force that overcomes the memory's
//! natural resistance. Suppressed memories may recover if the inhibitory
//! force is removed and enough time passes.

use std::fmt;

use crate::MemoryError;

pub mod decay;
pub mod suppression;

pub use decay::{MemoryDecay, DecayType};
pub use suppression::MemorySuppression;

/// Default decay rate for exponential forgetting.
pub const DEFAULT_DECAY_RATE: f64 = 0.02;
/// Default suppression force for active inhibition.
pub const DEFAULT_SUPPRESSION_FORCE: f64 = 0.1;
/// Minimum memory strength for a trace to be considered extant.
pub const EXISTENCE_THRESHOLD: f64 = 0.05;
/// Maximum rehearsal boost per access.
pub const MAX_REHEARSAL_BOOST: f64 = 0.2;

/// Creates a new memory decay model with the given rate and type.
///
/// # Errors
///
/// Returns [`MemoryError::OutOfRange`] if rate is outside [0.0, 1.0].
pub fn create_decay(rate: f64, decay_type: DecayType) -> Result<MemoryDecay, MemoryError> {
    MemoryDecay::new(rate, decay_type)
}

/// Creates a new memory suppression model.
///
/// # Errors
///
/// Returns [`MemoryError::OutOfRange`] if force or resistance is outside [0.0, 1.0].
pub fn create_suppression(force: f64, resistance: f64) -> Result<MemorySuppression, MemoryError> {
    MemorySuppression::new(force, resistance)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decay_creation_succeeds() {
        let d = create_decay(0.05, DecayType::Exponential).unwrap();
        assert!(!d.is_dissolved(0.5));
    }

    #[test]
    fn exponential_decay_reduces_strength() {
        let d = create_decay(0.1, DecayType::Exponential).unwrap();
        let remaining = d.apply_decay(0.5, 1.0);
        assert!(remaining < 1.0);
    }

    #[test]
    fn power_law_decay_rapid_initially() {
        let d = create_decay(0.1, DecayType::PowerLaw).unwrap();
        let short = d.apply_decay(0.1, 1.0);
        let long = d.apply_decay(1.0, 1.0);
        assert!(short > long);
    }

    #[test]
    fn suppression_reduces_net_strength() {
        let mut s = create_suppression(0.5, 0.3).unwrap();
        let net = s.net_strength(1.0);
        assert!(net < 1.0);
    }

    #[test]
    fn suppression_release_restores_strength() {
        let mut s = create_suppression(0.6, 0.4).unwrap();
        s.release(0.5);
        assert!(s.net_strength(1.0) > 0.0);
    }

    #[test]
    fn default_constants_valid() {
        assert!(DEFAULT_DECAY_RATE >= 0.0 && DEFAULT_DECAY_RATE <= 1.0);
        assert!(EXISTENCE_THRESHOLD >= 0.0);
    }
}
