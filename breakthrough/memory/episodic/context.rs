// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use crate::MemoryError;

use super::*;

/// EpisodicContext: Spatiotemporal and causal framing that binds events into narratives.
///
/// Context provides the scaffold on which episodic events hang. It encodes
/// the temporal frame (when), spatial frame (where), and causal chain (why)
/// that gives an event its meaning and distinguishes it from similar episodes.
#[derive(Debug, Clone, PartialEq)]
pub struct EpisodicContext {
    /// Strength of temporal framing (time-of-day, sequence, duration).
    pub temporal_frame: f64,
    /// Strength of spatial framing (location, environment, landmarks).
    pub spatial_frame: f64,
    /// Number of causally linked events in the chain.
    pub causal_chain: usize,
    /// Overall contextual strength in [0.0, 1.0].
    pub contextual_strength: f64,
    /// Maximum events this context can bind.
    pub capacity: usize,
    /// Abstract timestamp of context formation.
    pub formation_time: f64,
}

impl EpisodicContext {
    pub const MIN_FRAME: f64 = 0.0;
    pub const MAX_FRAME: f64 = 1.0;

    /// Creates a new EpisodicContext with the given framing strengths.
    ///
    /// # Errors
    ///
    /// Returns [`MemoryError::OutOfRange`] if any frame strength is outside [0.0, 1.0].
    pub fn new(temporal_frame: f64, spatial_frame: f64) -> Result<Self, MemoryError> {
        if !(Self::MIN_FRAME..=Self::MAX_FRAME).contains(&temporal_frame) {
            return Err(MemoryError::OutOfRange {
                field: "temporal_frame".into(),
                value: temporal_frame,
                min: Self::MIN_FRAME,
                max: Self::MAX_FRAME,
            });
        }
        if !(Self::MIN_FRAME..=Self::MAX_FRAME).contains(&spatial_frame) {
            return Err(MemoryError::OutOfRange {
                field: "spatial_frame".into(),
                value: spatial_frame,
                min: Self::MIN_FRAME,
                max: Self::MAX_FRAME,
            });
        }
        Ok(Self {
            temporal_frame,
            spatial_frame,
            causal_chain: 0,
            contextual_strength: 0.0,
            capacity: MAX_CONTEXT_EVENTS,
            formation_time: 0.0,
        })
    }

    /// Binds an event to this context, extending the causal chain.
    ///
    /// # Errors
    ///
    /// Returns [`MemoryError::CapacityExceeded`] if the context is at capacity.
    pub fn bind_event(&mut self, event_strength: f64) -> Result<(), MemoryError> {
        if self.causal_chain >= self.capacity {
            return Err(MemoryError::CapacityExceeded {
                max: self.capacity,
                attempted: self.causal_chain + 1,
            });
        }
        self.causal_chain += 1;
        self.contextual_strength = (self.contextual_strength + event_strength * 0.1).min(1.0);
        Ok(())
    }

    /// Extracts feature vectors from the context framing.
    ///
    /// Returns a vector combining temporal, spatial, and causal features.
    pub fn extract_features(&self) -> Vec<f64> {
        vec![
            self.temporal_frame,
            self.spatial_frame,
            self.causal_chain as f64 / self.capacity as f64,
            self.contextual_strength,
        ]
    }

    /// Computes temporal proximity to another context.
    ///
    /// Returns a similarity score in [0.0, 1.0].
    pub fn temporal_proximity(&self, other: &EpisodicContext) -> f64 {
        let diff = (self.temporal_frame - other.temporal_frame).abs();
        (1.0 - diff).clamp(0.0, 1.0)
    }

    /// Returns the number of causal links in this context.
    pub fn causal_links(&self) -> usize {
        self.causal_chain
    }

    /// Advances the context formation time.
    pub fn advance_time(&mut self, delta: f64) {
        self.formation_time += delta;
    }

    /// Validates the context state.
    pub fn validate(&self) -> Result<(), MemoryError> {
        Self::new(self.temporal_frame, self.spatial_frame)?;
        if self.causal_chain > self.capacity {
            return Err(MemoryError::CapacityExceeded {
                max: self.capacity,
                attempted: self.causal_chain,
            });
        }
        if !(0.0..=1.0).contains(&self.contextual_strength) {
            return Err(MemoryError::OutOfRange {
                field: "contextual_strength".into(),
                value: self.contextual_strength,
                min: 0.0,
                max: 1.0,
            });
        }
        Ok(())
    }
}

