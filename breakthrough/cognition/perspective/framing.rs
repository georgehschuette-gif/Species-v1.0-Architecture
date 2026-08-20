// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use super::*;

/// FrameType: The interpretive lens applied to perceptions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameType {
    /// Analytical decomposition into constituent parts.
    Analytic,
    /// Holistic assessment of the whole pattern.
    Holistic,
    /// Practical evaluation of utility and applicability.
    Pragmatic,
    /// Critical assessment of assumptions and validity.
    Critical,
}

/// CognitiveFraming: The interpretive frame applied to perceptions.
///
/// Framing determines how raw perceptual data is interpreted
/// within the cognitive system. Different frames produce
/// different cognitive outputs from the same input. The
/// strength parameter determines how firmly the frame is held.
///
/// # Fields
/// - `frame_type`: The type of interpretive frame being applied.
/// - `strength`: How firmly the frame is held, in [0.0, 1.0].
///
/// # Example
/// ```
/// use breakthrough::cognition::perspective::{CognitiveFraming, FrameType};
///
/// let mut frame = CognitiveFraming::new(FrameType::Analytic, 0.7).expect("valid parameters");
/// frame.strengthen(0.1);
/// assert!(frame.strength() > 0.7);
/// ```
pub struct CognitiveFraming {
    /// The type of interpretive frame being applied.
    pub frame_type: FrameType,
    /// How firmly the frame is held, in [0.0, 1.0].
    pub strength: f64,
}

impl CognitiveFraming {
    /// Creates a new `CognitiveFraming` with the given frame type and strength.
    ///
    /// # Errors
    /// Returns [`CognitionError::OutOfRange`] if `strength` is outside [0.0, 1.0].
    pub fn new(frame_type: FrameType, strength: f64) -> Result<Self, CognitionError> {
        if !(0.0..=1.0).contains(&strength) {
            return Err(CognitionError::OutOfRange {
                field: "strength".to_string(),
                value: strength,
                min: 0.0,
                max: 1.0,
            });
        }
        Ok(Self {
            frame_type,
            strength,
        })
    }

    /// Strengthens the framing by the given amount, capped at 1.0.
    ///
    /// # Errors
    /// Returns [`CognitionError::OutOfRange`] if the increment is negative.
    pub fn strengthen(&mut self, increment: f64) -> Result<f64, CognitionError> {
        if increment < 0.0 {
            return Err(CognitionError::OutOfRange {
                field: "increment".to_string(),
                value: increment,
                min: 0.0,
                max: f64::INFINITY,
            });
        }
        self.strength = (self.strength + increment).min(1.0);
        Ok(self.strength)
    }

    /// Weakens the framing by the given amount, floored at 0.0.
    ///
    /// # Errors
    /// Returns [`CognitionError::OutOfRange`] if the decrement is negative.
    pub fn weaken(&mut self, decrement: f64) -> Result<f64, CognitionError> {
        if decrement < 0.0 {
            return Err(CognitionError::OutOfRange {
                field: "decrement".to_string(),
                value: decrement,
                min: 0.0,
                max: f64::INFINITY,
            });
        }
        self.strength = (self.strength - decrement).max(0.0);
        Ok(self.strength)
    }

    /// Returns the current frame strength.
    pub fn strength(&self) -> f64 {
        self.strength
    }

    /// Changes the frame type.
    pub fn set_frame_type(&mut self, new_type: FrameType) {
        self.frame_type = new_type;
    }

    /// Computes a framing score that measures how strongly the frame
    /// influences interpretation, factoring in both strength and
    /// the inherent rigidity of the frame type.
    pub fn influence_score(&self) -> f64 {
        let type_factor = match self.frame_type {
            FrameType::Analytic => 0.8,
            FrameType::Holistic => 0.6,
            FrameType::Pragmatic => 0.9,
            FrameType::Critical => 0.7,
        };
        self.strength * type_factor
    }

    /// Validates all field values are within valid ranges.
    pub fn validate(&self) -> Result<(), CognitionError> {
        if !(0.0..=1.0).contains(&self.strength) {
            return Err(CognitionError::OutOfRange {
                field: "strength".to_string(),
                value: self.strength,
                min: 0.0,
                max: 1.0,
            });
        }
        Ok(())
    }
}
