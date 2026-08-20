// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use super::*;

/// IdentityContinuity: Maintaining persistent identity across state changes.
///
/// Continuity ensures that a concept or system retains its
/// core identity despite changes in its observable properties.
/// The continuity strength resists drift, while the memory
/// window determines how far back identity is anchored.
///
/// # Fields
/// - `continuity_strength`: Resistance to identity drift, in [0.0, 1.0].
/// - `memory_window`: Temporal span over which identity is preserved, in > 0.0.
///
/// # Example
/// ```
/// use breakthrough::cognition::identity::IdentityContinuity;
///
/// let mut continuity = IdentityContinuity::new(0.8, 10.0).expect("valid parameters");
/// let preserved = continuity.maintain(0.6).expect("maintained identity");
/// assert!(preserved > 0.5);
/// ```
pub struct IdentityContinuity {
    /// Resistance to identity drift, in [0.0, 1.0].
    pub continuity_strength: f64,
    /// Temporal span over which identity is preserved, in > 0.0.
    pub memory_window: f64,
}

impl IdentityContinuity {
    /// Creates a new `IdentityContinuity` with the given strength and window.
    ///
    /// # Errors
    /// Returns [`CognitionError::OutOfRange`] if `continuity_strength` is outside [0.0, 1.0]
    /// or `memory_window` is not positive.
    pub fn new(
        continuity_strength: f64,
        memory_window: f64,
    ) -> Result<Self, CognitionError> {
        if memory_window <= 0.0 {
            return Err(CognitionError::OutOfRange {
                field: "memory_window".to_string(),
                value: memory_window,
                min: 0.0,
                max: f64::INFINITY,
            });
        }
        Ok(Self {
            continuity_strength,
            memory_window,
        })
    }

    /// Maintains identity for an observed value, preventing drift.
    ///
    /// If the observed value deviates from the anchored identity by
    /// more than the allowed drift threshold, the continuity strength
    /// pulls it back toward the anchor.
    ///
    /// # Errors
    /// Returns [`CognitionError::OutOfRange`] if `observed` is outside [0.0, 1.0].
    pub fn maintain(&self, observed: f64) -> Result<f64, CognitionError> {
        if !(0.0..=1.0).contains(&observed) {
            return Err(CognitionError::OutOfRange {
                field: "observed".to_string(),
                value: observed,
                min: 0.0,
                max: 1.0,
            });
        }
        let drift = observed - 0.5;
        let correction = drift * self.continuity_strength;
        let anchored = observed - correction;
        Ok(anchored.clamp(0.0, 1.0))
    }

    /// Checks whether the identity has drifted beyond the acceptable threshold.
    pub fn has_drifted(&self, observed: f64, threshold: f64) -> Result<bool, CognitionError> {
        if !(0.0..=1.0).contains(&threshold) {
            return Err(CognitionError::OutOfRange {
                field: "threshold".to_string(),
                value: threshold,
                min: 0.0,
                max: 1.0,
            });
        }
        let maintained = self.maintain(observed)?;
        Ok((observed - maintained).abs() > threshold)
    }

    /// Computes the identity coherence score.
    pub fn coherence(&self) -> f64 {
        self.continuity_strength * (1.0 - 1.0 / (1.0 + self.memory_window))
    }

    /// Updates the continuity strength.
    ///
    /// # Errors
    /// Returns [`CognitionError::OutOfRange`] if the new strength is outside [0.0, 1.0].
    pub fn set_strength(&mut self, new_strength: f64) -> Result<(), CognitionError> {
        if !(0.0..=1.0).contains(&new_strength) {
            return Err(CognitionError::OutOfRange {
                field: "continuity_strength".to_string(),
                value: new_strength,
                min: 0.0,
                max: 1.0,
            });
        }
        self.continuity_strength = new_strength;
        Ok(())
    }

    /// Validates all field values are within valid ranges.
    pub fn validate(&self) -> Result<(), CognitionError> {
        if !(0.0..=1.0).contains(&self.continuity_strength) {
            return Err(CognitionError::OutOfRange {
                field: "continuity_strength".to_string(),
                value: self.continuity_strength,
                min: 0.0,
                max: 1.0,
            });
        }
        if self.memory_window <= 0.0 {
            return Err(CognitionError::OutOfRange {
                field: "memory_window".to_string(),
                value: self.memory_window,
                min: 0.0,
                max: f64::INFINITY,
            });
        }
        Ok(())
    }
}

impl fmt::Debug for IdentityContinuity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("IdentityContinuity")
            .field("continuity_strength", &self.continuity_strength)
            .field("memory_window", &self.memory_window)
            .finish()
    }
}