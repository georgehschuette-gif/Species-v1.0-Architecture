// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use super::*;

/// SelfRegulate: Adjusts cognitive processes based on self-monitoring.
///
/// Self-regulation uses monitoring data to modulate cognitive
/// parameters dynamically. Stronger regulation is applied
/// when quality drops, while weaker regulation preserves
/// system responsiveness.
///
/// # Fields
/// - `regulation_strength`: Maximum adjustment magnitude, in [0.0, 1.0].
/// - `adaptation_rate`: Speed of regulatory response, in [0.0, 1.0].
///
/// # Example
/// ```
/// use breakthrough::cognition::metacognition::SelfRegulate;
///
/// let mut regulator = SelfRegulate::new(0.5, 0.2).expect("valid parameters");
/// let adjustment = regulator.regulate(0.3, 0.8).expect("regulate succeeded");
/// assert!(adjustment > 0.0);
/// ```
pub struct SelfRegulate {
    /// Maximum adjustment magnitude, in [0.0, 1.0].
    pub regulation_strength: f64,
    /// Speed of regulatory response, in [0.0, 1.0].
    pub adaptation_rate: f64,
}

impl SelfRegulate {
    /// Creates a new `SelfRegulate` with the given strength and rate.
    ///
    /// # Errors
    /// Returns [`CognitionError::OutOfRange`] if either parameter is outside [0.0, 1.0].
    pub fn new(
        regulation_strength: f64,
        adaptation_rate: f64,
    ) -> Result<Self, CognitionError> {
        if !(0.0..=1.0).contains(&regulation_strength) {
            return Err(CognitionError::OutOfRange {
                field: "regulation_strength".to_string(),
                value: regulation_strength,
                min: 0.0,
                max: 1.0,
            });
        }
        if !(0.0..=1.0).contains(&adaptation_rate) {
            return Err(CognitionError::OutOfRange {
                field: "adaptation_rate".to_string(),
                value: adaptation_rate,
                min: 0.0,
                max: 1.0,
            });
        }
        Ok(Self {
            regulation_strength,
            adaptation_rate,
        })
    }

    /// Regulates a cognitive parameter based on the quality score.
    ///
    /// If quality is below 0.5, the parameter is pushed upward
    /// by an amount proportional to the deficit. If quality is
    /// above 0.8, the parameter is gently restrained.
    ///
    /// # Errors
    /// Returns [`CognitionError::OutOfRange`] if `target` is outside [0.0, 1.0].
    pub fn regulate(&self, target: f64, quality: f64) -> Result<f64, CognitionError> {
        if !(0.0..=1.0).contains(&target) {
            return Err(CognitionError::OutOfRange {
                field: "target".to_string(),
                value: target,
                min: 0.0,
                max: 1.0,
            });
        }
        if !(0.0..=1.0).contains(&quality) {
            return Err(CognitionError::OutOfRange {
                field: "quality".to_string(),
                value: quality,
                min: 0.0,
                max: 1.0,
            });
        }
        let adjustment = if quality < 0.5 {
            // Quality is low: push parameter up
            let deficit = 0.5 - quality;
            self.regulation_strength * deficit * self.adaptation_rate
        } else if quality > 0.8 {
            // Quality is high: gentle restraint
            let surplus = quality - 0.8;
            -self.regulation_strength * surplus * self.adaptation_rate * 0.5
        } else {
            // Quality is adequate: no adjustment needed
            0.0
        };
        let new_value = (target + adjustment).clamp(0.0, 1.0);
        Ok(new_value)
    }

    /// Computes the regulation delta for a given target and quality pair.
    pub fn regulation_delta(&self, target: f64, quality: f64) -> Result<f64, CognitionError> {
        let new_value = self.regulate(target, quality)?;
        Ok(new_value - target)
    }

    /// Adjusts the regulation strength.
    ///
    /// # Errors
    /// Returns [`CognitionError::OutOfRange`] if the new strength is outside [0.0, 1.0].
    pub fn set_strength(&mut self, new_strength: f64) -> Result<(), CognitionError> {
        if !(0.0..=1.0).contains(&new_strength) {
            return Err(CognitionError::OutOfRange {
                field: "regulation_strength".to_string(),
                value: new_strength,
                min: 0.0,
                max: 1.0,
            });
        }
        self.regulation_strength = new_strength;
        Ok(())
    }

    /// Adjusts the adaptation rate.
    ///
    /// # Errors
    /// Returns [`CognitionError::OutOfRange`] if the new rate is outside [0.0, 1.0].
    pub fn set_adaptation_rate(&mut self, new_rate: f64) -> Result<(), CognitionError> {
        if !(0.0..=1.0).contains(&new_rate) {
            return Err(CognitionError::OutOfRange {
                field: "adaptation_rate".to_string(),
                value: new_rate,
                min: 0.0,
                max: 1.0,
            });
        }
        self.adaptation_rate = new_rate;
        Ok(())
    }

    /// Validates all field values are within valid ranges.
    pub fn validate(&self) -> Result<(), CognitionError> {
        if !(0.0..=1.0).contains(&self.regulation_strength) {
            return Err(CognitionError::OutOfRange {
                field: "regulation_strength".to_string(),
                value: self.regulation_strength,
                min: 0.0,
                max: 1.0,
            });
        }
        if !(0.0..=1.0).contains(&self.adaptation_rate) {
            return Err(CognitionError::OutOfRange {
                field: "adaptation_rate".to_string(),
                value: self.adaptation_rate,
                min: 0.0,
                max: 1.0,
            });
        }
        Ok(())
    }
}

impl fmt::Debug for SelfRegulate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SelfRegulate")
            .field("regulation_strength", &self.regulation_strength)
            .field("adaptation_rate", &self.adaptation_rate)
            .finish()
    }
}