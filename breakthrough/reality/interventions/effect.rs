// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use crate::RealityError;

/// Effect: The expected or actual effect of an intervention.
///
/// Effects quantify the change induced by an intervention, both in
/// magnitude and direction.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Effect {
    /// Magnitude of the effect (absolute change).
    pub magnitude: f64,
    /// Direction of the effect (positive or negative change).
    pub direction: EffectDirection,
    /// Duration over which the effect persists.
    pub duration: f64,
    /// Certainty of this effect estimate.
    pub certainty: f64,
}

impl Effect {
    /// Minimum valid magnitude.
    pub const MIN_MAGNITUDE: f64 = 0.0;
    /// Minimum valid duration.
    pub const MIN_DURATION: f64 = 0.0;

    /// Creates a new effect.
    ///
    /// # Errors
    /// Returns `RealityError::InvalidMeasurement` if parameters are invalid.
    pub fn new(
        magnitude: f64,
        direction: EffectDirection,
        duration: f64,
        certainty: f64,
    ) -> Result<Self, RealityError> {
        if magnitude < Self::MIN_MAGNITUDE {
            return Err(RealityError::InvalidMeasurement {
                value: magnitude,
                reason: "magnitude cannot be negative".to_string(),
            });
        }
        if duration < Self::MIN_DURATION {
            return Err(RealityError::InvalidMeasurement {
                value: duration,
                reason: "duration cannot be negative".to_string(),
            });
        }
        if certainty < 0.0 || certainty > 1.0 {
            return Err(RealityError::OutOfRange {
                field: "certainty".to_string(),
                value: certainty,
                min: 0.0,
                max: 1.0,
            });
        }
        Ok(Self {
            magnitude,
            direction,
            duration,
            certainty,
        })
    }

    /// Returns the signed magnitude (positive or negative based on direction).
    pub fn signed_magnitude(&self) -> f64 {
        match self.direction {
            EffectDirection::Positive => self.magnitude,
            EffectDirection::Negative => -self.magnitude,
            EffectDirection::Neutral => 0.0,
        }
    }

    /// Returns the rate of change per unit time.
    pub fn rate(&self) -> f64 {
        if self.duration <= f64::EPSILON {
            f64::INFINITY
        } else {
            self.magnitude / self.duration
        }
    }

    /// Combines this effect with another (assuming sequential application).
    pub fn combine(&self, other: &Effect) -> Effect {
        let total_magnitude = self.magnitude + other.magnitude;
        let combined_direction = if total_magnitude > f64::EPSILON {
            EffectDirection::Positive
        } else if total_magnitude < -f64::EPSILON {
            EffectDirection::Negative
        } else {
            EffectDirection::Neutral
        };
        let combined_certainty = (self.certainty + other.certainty) / 2.0;
        Effect {
            magnitude: total_magnitude.abs(),
            direction: combined_direction,
            duration: self.duration + other.duration,
            certainty: combined_certainty,
        }
    }
}

/// EffectDirection: Whether an effect increases or decreases a quantity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EffectDirection {
    /// The effect increases the target quantity.
    Positive,
    /// The effect decreases the target quantity.
    Negative,
    /// The effect has no net directional change.
    Neutral,
}

impl EffectDirection {
    /// Returns the string label of this direction.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Positive => "positive",
            Self::Negative => "negative",
            Self::Neutral => "neutral",
        }
    }

    /// Returns the opposite direction.
    pub fn opposite(&self) -> Self {
        match self {
            Self::Positive => Self::Negative,
            Self::Negative => Self::Positive,
            Self::Neutral => Self::Neutral,
        }
    }

    /// Returns the multiplier sign (+1.0 or -1.0).
    pub fn sign(&self) -> f64 {
        match self {
            Self::Positive => 1.0,
            Self::Negative => -1.0,
            Self::Neutral => 0.0,
        }
    }
}

impl std::fmt::Display for EffectDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Default for EffectDirection {
    fn default() -> Self {
        Self::Neutral
    }
}

