// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use super::*;

/// TemporalRecency: Time-decayed relevance of cognitive content.
pub struct TemporalRecency {
    pub timestamp: f64,
    pub half_life: f64,
    pub current_relevance: f64,
}

impl TemporalRecency {
    pub fn new(timestamp: f64, half_life: f64) -> Result<Self, TemporalError> {
        if !timestamp.is_finite() {
            return Err(TemporalError::InvalidTimestamp { timestamp });
        }
        if half_life <= 0.0 {
            return Err(TemporalError::InvalidHalfLife { half_life });
        }
        Ok(Self { timestamp, half_life, current_relevance: 1.0 })
    }

    pub fn relevance_at(&self, current_time: f64) -> f64 {
        let age = current_time - self.timestamp;
        if age <= 0.0 { return 1.0; }
        0.5f64.powf(age / self.half_life)
    }

    pub fn update(&mut self, current_time: f64) {
        self.current_relevance = self.relevance_at(current_time);
    }

    pub fn is_fresh(&self, current_time: f64) -> bool {
        self.relevance_at(current_time) > 0.5
    }
}
