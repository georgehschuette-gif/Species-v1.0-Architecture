// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use crate::MemoryError;

use super::*;

/// EpisodicEvent: A single autobiographical memory of an experienced occurrence.
///
/// Each event captures a moment in time with associated sensory data, emotional
/// coloring, and vividness. Events age naturally, losing detail while potentially
/// gaining semantic coherence through consolidation.
#[derive(Debug, Clone, PartialEq)]
pub struct EpisodicEvent {
    /// Current vividness of the memory in [0.0, 1.0].
    pub vividness: f64,
    /// Emotional valence of the event: negative to positive in [-1.0, 1.0].
    pub emotional_valence: f64,
    /// Rate at which vividness decays per time unit.
    pub decay_rate: f64,
    /// Age of the event since original encoding.
    pub age: f64,
    /// Degree of semantic integration (post-consolidation) in [0.0, 1.0].
    pub coherence: f64,
    /// Number of times this event has been reconsolidated.
    pub reconsolidation_count: usize,
    /// Sensory features captured at encoding.
    pub sensory_snapshot: Vec<f64>,
    /// Optional narrative description.
    pub narrative: String,
}

impl EpisodicEvent {
    pub const MIN_VIVIDNESS: f64 = 0.0;
    pub const MAX_VIVIDNESS: f64 = 1.0;
    pub const MIN_VALENCE: f64 = -1.0;
    pub const MAX_VALENCE: f64 = 1.0;
    pub const MIN_DECAY: f64 = 0.0;
    pub const MAX_DECAY: f64 = 1.0;

    /// Creates a new EpisodicEvent with the given parameters.
    ///
    /// # Errors
    ///
    /// Returns [`MemoryError::OutOfRange`] if any parameter is outside its valid range.
    pub fn new(
        vividness: f64,
        emotional_valence: f64,
        decay_rate: f64,
    ) -> Result<Self, MemoryError> {
        if !(Self::MIN_VIVIDNESS..=Self::MAX_VIVIDNESS).contains(&vividness) {
            return Err(MemoryError::OutOfRange {
                field: "vividness".into(),
                value: vividness,
                min: Self::MIN_VIVIDNESS,
                max: Self::MAX_VIVIDNESS,
            });
        }
        if !(Self::MIN_VALENCE..=Self::MAX_VALENCE).contains(&emotional_valence) {
            return Err(MemoryError::OutOfRange {
                field: "emotional_valence".into(),
                value: emotional_valence,
                min: Self::MIN_VALENCE,
                max: Self::MAX_VALENCE,
            });
        }
        if !(Self::MIN_DECAY..=Self::MAX_DECAY).contains(&decay_rate) {
            return Err(MemoryError::OutOfRange {
                field: "decay_rate".into(),
                value: decay_rate,
                min: Self::MIN_DECAY,
                max: Self::MAX_DECAY,
            });
        }
        Ok(Self {
            vividness,
            emotional_valence,
            decay_rate,
            age: 0.0,
            coherence: 0.0,
            reconsolidation_count: 0,
            sensory_snapshot: Vec::new(),
            narrative: String::new(),
        })
    }

    /// Ages the event by delta time, reducing vividness.
    pub fn age(&mut self, delta: f64) {
        if delta < 0.0 {
            return;
        }
        self.age += delta;
        let decay = (1.0 - self.decay_rate).powf(delta);
        self.vividness *= decay;
        self.vividness = self.vividness.clamp(Self::MIN_VIVIDNESS, Self::MAX_VIVIDNESS);
    }

    /// Embellishes the event by adding detail and shifting valence toward the given direction.
    ///
    /// # Errors
    ///
    /// Returns [`MemoryError::OutOfRange`] if direction is outside [-1.0, 1.0].
    pub fn embellish(&mut self, detail_amount: f64, direction: f64) -> Result<(), MemoryError> {
        if !(-1.0..=1.0).contains(&direction) {
            return Err(MemoryError::OutOfRange {
                field: "direction".into(),
                value: direction,
                min: -1.0,
                max: 1.0,
            });
        }
        self.vividness = (self.vividness + detail_amount * 0.1).min(Self::MAX_VIVIDNESS);
        self.emotional_valence = (self.emotional_valence + direction * 0.05).clamp(
            Self::MIN_VALENCE,
            Self::MAX_VALENCE,
        );
        Ok(())
    }

    /// Consolidates the event, converting vivid episodic detail into stable semantic coherence.
    ///
    /// # Errors
    ///
    /// Returns [`MemoryError::ThresholdNotMet`] if vividness is below threshold.
    pub fn consolidate(&mut self, delta: f64) -> Result<(), MemoryError> {
        if self.vividness < 0.3 {
            return Err(MemoryError::ThresholdNotMet {
                threshold: 0.3,
                actual: self.vividness,
            });
        }
        self.coherence = (self.coherence + delta * 0.05).min(1.0);
        self.vividness = (self.vividness - delta * 0.02).max(0.0);
        Ok(())
    }

    /// Computes episodic similarity by comparing sensory snapshots.
    pub fn similarity(&self, other: &EpisodicEvent) -> f64 {
        if self.sensory_snapshot.is_empty() || other.sensory_snapshot.is_empty() {
            return 0.0;
        }
        let min_len = self
            .sensory_snapshot
            .len()
            .min(other.sensory_snapshot.len());
        let dot: f64 = self
            .sensory_snapshot
            .iter()
            .zip(other.sensory_snapshot.iter())
            .take(min_len)
            .map(|(a, b)| (a - b).powi(2))
            .sum();
        let dist = (dot / min_len as f64).sqrt();
        (1.0 - dist).clamp(0.0, 1.0)
    }

    /// Returns the emotional tone category of this event.
    pub fn emotional_tone(&self) -> &'static str {
        if self.emotional_valence > 0.3 {
            "positive"
        } else if self.emotional_valence < -0.3 {
            "negative"
        } else {
            "neutral"
        }
    }

    /// Sets the sensory snapshot captured during encoding.
    pub fn set_sensory_snapshot(&mut self, snapshot: Vec<f64>) {
        self.sensory_snapshot = snapshot;
    }

    /// Sets the narrative description of the event.
    pub fn set_narrative(&mut self, narrative: impl Into<String>) {
        self.narrative = narrative.into();
    }

    /// Returns whether the event retains enough coherence to be retrieved.
    pub fn is_coherent(&self) -> bool {
        self.coherence >= MIN_COHERENCE || self.vividness >= MIN_COHERENCE
    }

    /// Validates the event state.
    pub fn validate(&self) -> Result<(), MemoryError> {
        Self::new(self.vividness, self.emotional_valence, self.decay_rate)?;
        if self.age < 0.0 {
            return Err(MemoryError::OutOfRange {
                field: "age".into(),
                value: self.age,
                min: 0.0,
                max: f64::INFINITY,
            });
        }
        if !(0.0..=1.0).contains(&self.coherence) {
            return Err(MemoryError::OutOfRange {
                field: "coherence".into(),
                value: self.coherence,
                min: 0.0,
                max: 1.0,
            });
        }
        Ok(())
    }
}

