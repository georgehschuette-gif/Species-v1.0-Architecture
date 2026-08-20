// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

//! Episodic Memory: Autobiographical event memory with spatiotemporal context.
//!
//! Episodic memory stores personally experienced events as rich, contextualized
//! episodes. Each episode captures the what, where, when, and emotional valence
//! of an experience, enabling mental time travel and future projection.
//!
//! # Components
//!
//! - **EpisodicEvent** — A single memory of an experienced occurrence, with
//!   sensory snapshot, emotional tone, and vividness metrics.
//! - **EpisodicContext** — The spatiotemporal and causal framing that binds
//!   events into coherent narratives.
//!
//! # Consolidation
//!
//! Fresh episodes are initially vivid but fragile. Over time, they consolidate
//! into stable representations, becoming less detailed but more semantically
//! integrated. Reconsolidation allows reactivation to update or embellish the
//! episode.
//!
//! # Examples
//!
//! ```
//! use breakthrough::memory::episodic::{EpisodicEvent, EpisodicContext};
//!
//! let mut event = EpisodicEvent::new(0.9, 0.7, 0.8).expect("valid event");
//! event.age(2.0);
//! assert!(event.vividness < 0.8);
//!
//! let mut ctx = EpisodicContext::new(0.5, 0.3).expect("valid context");
//! ctx.bind_event(0.9).unwrap();
//! assert!(ctx.causal_chain >= 1);
//! ```

use std::fmt;

use crate::MemoryError;

pub mod event;
pub mod context;

pub use event::EpisodicEvent;
pub use context::EpisodicContext;

/// Default emotional valence bias for novel events.
pub const DEFAULT_VALENCE_BIAS: f64 = 0.5;
/// Rate at which episodic vividness decays per time unit.
pub const DEFAULT_VIVIDNESS_DECAY: f64 = 0.03;
/// Minimum context strength for an episode to be considered coherent.
pub const MIN_COHERENCE: f64 = 0.2;
/// Maximum number of events that can be bound to a single context.
pub const MAX_CONTEXT_EVENTS: usize = 500;

/// Creates a new episodic event with the given vividness, valence, and decay rate.
///
/// # Errors
///
/// Returns [`MemoryError::OutOfRange`] if any parameter is outside [0.0, 1.0].
pub fn create_event(
    vividness: f64,
    emotional_valence: f64,
    decay_rate: f64,
) -> Result<EpisodicEvent, MemoryError> {
    EpisodicEvent::new(vividness, emotional_valence, decay_rate)
}

/// Creates a new episodic context with the given temporal and spatial framing strengths.
///
/// # Errors
///
/// Returns [`MemoryError::OutOfRange`] if any parameter is outside [0.0, 1.0].
pub fn create_context(
    temporal_frame: f64,
    spatial_frame: f64,
) -> Result<EpisodicContext, MemoryError> {
    EpisodicContext::new(temporal_frame, spatial_frame)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn event_creation_succeeds() {
        let e = create_event(0.8, 0.6, 0.05).unwrap();
        assert!(e.is_coherent());
        assert_eq!(e.emotional_valence, 0.6);
    }

    #[test]
    fn event_ages_and_loses_vividness() {
        let mut e = create_event(1.0, 0.5, DEFAULT_VIVIDNESS_DECAY).unwrap();
        e.age(1.0);
        assert!(e.vividness < 1.0);
    }

    #[test]
    fn event_consolidation_improves_coherence() {
        let mut e = create_event(0.5, 0.5, 0.02).unwrap();
        let before = e.coherence;
        e.consolidate(1.0).unwrap();
        assert!(e.coherence >= before);
    }

    #[test]
    fn context_binds_event() {
        let mut ctx = create_context(0.6, 0.4).unwrap();
        ctx.bind_event(0.9).unwrap();
        assert_eq!(ctx.causal_chain, 1);
    }

    #[test]
    fn context_temporal_proximity() {
        let ctx = create_context(0.7, 0.3).unwrap();
        assert!(ctx.temporal_frame >= 0.0 && ctx.temporal_frame <= 1.0);
    }

    #[test]
    fn default_constants_valid() {
        assert!(DEFAULT_VALENCE_BIAS >= 0.0 && DEFAULT_VALENCE_BIAS <= 1.0);
        assert!(DEFAULT_VIVIDNESS_DECAY >= 0.0);
        assert!(MAX_CONTEXT_EVENTS > 0);
    }
}
