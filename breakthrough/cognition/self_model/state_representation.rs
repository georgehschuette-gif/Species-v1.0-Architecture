// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use super::*;

/// StateRepresentation: The system's internal model of its own state.
///
/// State representation maintains a vector of values describing
/// the system's current state along with a confidence measure.
/// The confidence indicates how accurately the state vector
/// reflects reality and can be updated as new observations arrive.
///
/// # Fields
/// - `state_vector`: The current state as a vector of values.
/// - `confidence`: How confident the system is in its state model, in [0.0, 1.0].
///
/// # Example
/// ```
/// use breakthrough::cognition::self_model::StateRepresentation;
///
/// let mut state = StateRepresentation::new(vec![0.1, 0.5, 0.9], 0.8).expect("valid");
/// state.update(0, 0.3).expect("update succeeded");
/// assert_eq!(state.get(0), 0.3);
/// ```
pub struct StateRepresentation {
    /// The current state as a vector of values.
    pub state_vector: Vec<f64>,
    /// How confident the system is in its state model, in [0.0, 1.0].
    pub confidence: f64,
}

impl StateRepresentation {
    /// Creates a new `StateRepresentation` with the given vector and confidence.
    ///
    /// # Errors
    /// Returns [`CognitionError::MissingInput`] if the state vector is empty.
    /// Returns [`CognitionError::OutOfRange`] if confidence is outside [0.0, 1.0].
    pub fn new(state_vector: Vec<f64>, confidence: f64) -> Result<Self, CognitionError> {
        if state_vector.is_empty() {
            return Err(CognitionError::MissingInput(
                "state_vector must not be empty".to_string(),
            ));
        }
        for (i, &val) in state_vector.iter().enumerate() {
            if val.is_nan() || val.is_infinite() {
                return Err(CognitionError::InvalidState(format!(
                    "state_vector[{}] is NaN or infinite",
                    i
                )));
            }
        }
        if !(0.0..=1.0).contains(&confidence) {
            return Err(CognitionError::OutOfRange {
                field: "confidence".to_string(),
                value: confidence,
                min: 0.0,
                max: 1.0,
            });
        }
        Ok(Self {
            state_vector,
            confidence,
        })
    }

    /// Updates the value at the given index.
    ///
    /// # Errors
    /// Returns [`CognitionError::OutOfRange`] if index is out of bounds
    /// or `value` is NaN or infinite.
    pub fn update(&mut self, index: usize, value: f64) -> Result<(), CognitionError> {
        if index >= self.state_vector.len() {
            return Err(CognitionError::OutOfRange {
                field: format!("state_vector[{}]", index),
                value: index as f64,
                min: 0.0,
                max: (self.state_vector.len() - 1) as f64,
            });
        }
        if value.is_nan() || value.is_infinite() {
            return Err(CognitionError::InvalidState(format!(
                "value {} is NaN or infinite",
                value
            )));
        }
        self.state_vector[index] = value;
        Ok(())
    }

    /// Returns the value at the given index.
    pub fn get(&self, index: usize) -> f64 {
        self.state_vector[index]
    }

    /// Returns the dimensionality of the state vector.
    pub fn dimensionality(&self) -> usize {
        self.state_vector.len()
    }

    /// Updates the confidence level.
    ///
    /// # Errors
    /// Returns [`CognitionError::OutOfRange`] if confidence is outside [0.0, 1.0].
    pub fn set_confidence(&mut self, new_confidence: f64) -> Result<(), CognitionError> {
        if !(0.0..=1.0).contains(&new_confidence) {
            return Err(CognitionError::OutOfRange {
                field: "confidence".to_string(),
                value: new_confidence,
                min: 0.0,
                max: 1.0,
            });
        }
        self.confidence = new_confidence;
        Ok(())
    }

    /// Returns the current confidence level.
    pub fn confidence(&self) -> f64 {
        self.confidence
    }

    /// Computes the L2 norm of the state vector.
    pub fn norm(&self) -> f64 {
        self.state_vector
            .iter()
            .map(|&v| v * v)
            .sum::<f64>()
            .sqrt()
    }

    /// Normalizes the state vector in-place to unit length.
    ///
    /// If the norm is zero, the vector is left unchanged.
    pub fn normalize(&mut self) {
        let n = self.norm();
        if n > 0.0 {
            for val in &mut self.state_vector {
                *val /= n;
            }
        }
    }

    /// Validates all field values are within valid ranges.
    pub fn validate(&self) -> Result<(), CognitionError> {
        if self.state_vector.is_empty() {
            return Err(CognitionError::MissingInput(
                "state_vector must not be empty".to_string(),
            ));
        }
        for (i, &val) in self.state_vector.iter().enumerate() {
            if val.is_nan() || val.is_infinite() {
                return Err(CognitionError::InvalidState(format!(
                    "state_vector[{}] is NaN or infinite",
                    i
                )));
            }
        }
        if !(0.0..=1.0).contains(&self.confidence) {
            return Err(CognitionError::OutOfRange {
                field: "confidence".to_string(),
                value: self.confidence,
                min: 0.0,
                max: 1.0,
            });
        }
        Ok(())
    }
}

impl fmt::Debug for StateRepresentation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("StateRepresentation")
            .field("state_vector", &self.state_vector)
            .field("confidence", &self.confidence)
            .finish()
    }
}