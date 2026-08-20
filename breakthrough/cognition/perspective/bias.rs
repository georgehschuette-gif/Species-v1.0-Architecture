// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use super::*;

/// PerspectiveBias: Systematic distortions introduced by viewpoint.
///
/// Bias represents the systematic skew in cognitive processing
/// that arises from the system's current perspective. It can
/// amplify or suppress certain perceptual dimensions, distorting
/// the cognitive output.
///
/// # Fields
/// - `bias_vector`: Directional weights showing which dimensions are amplified, values in [-1.0, 1.0].
/// - `magnitude`: Overall strength of the bias, in [0.0, 1.0].
///
/// # Example
/// ```
/// use breakthrough::cognition::perspective::PerspectiveBias;
///
/// let bias = PerspectiveBias::new(vec![0.5, -0.3, 0.1], 0.4).expect("valid parameters");
/// assert!(!bias.is_neutral());
/// ```
pub struct PerspectiveBias {
    /// Directional weights showing which dimensions are amplified, values in [-1.0, 1.0].
    pub bias_vector: Vec<f64>,
    /// Overall strength of the bias, in [0.0, 1.0].
    pub magnitude: f64,
}

impl PerspectiveBias {
    /// Creates a new `PerspectiveBias` with the given vector and magnitude.
    ///
    /// # Errors
    /// Returns [`CognitionError::MissingInput`] if the bias vector is empty.
    /// Returns [`CognitionError::OutOfRange`] if any vector component is outside [-1.0, 1.0]
    /// or if `magnitude` is outside [0.0, 1.0].
    pub fn new(bias_vector: Vec<f64>, magnitude: f64) -> Result<Self, CognitionError> {
        if bias_vector.is_empty() {
            return Err(CognitionError::MissingInput(
                "bias_vector must not be empty".to_string(),
            ));
        }
        for (i, &component) in bias_vector.iter().enumerate() {
            if component < -1.0 || component > 1.0 {
                return Err(CognitionError::OutOfRange {
                    field: format!("bias_vector[{}]", i),
                    value: component,
                    min: -1.0,
                    max: 1.0,
                });
            }
        }
        if !(0.0..=1.0).contains(&magnitude) {
            return Err(CognitionError::OutOfRange {
                field: "magnitude".to_string(),
                value: magnitude,
                min: 0.0,
                max: 1.0,
            });
        }
        Ok(Self {
            bias_vector,
            magnitude,
        })
    }

    /// Applies the bias to a set of perceptual values.
    ///
    /// Each dimension is scaled by `1.0 + bias_vector[i] * magnitude`,
    /// then clamped to [0.0, 1.0].
    ///
    /// # Errors
    /// Returns [`CognitionError::OutOfRange`] if dimensions don't match or values are out of bounds.
    pub fn apply_to(&self, values: &[f64]) -> Result<Vec<f64>, CognitionError> {
        if values.len() != self.bias_vector.len() {
            return Err(CognitionError::IncompatibleConcepts {
                reason: format!(
                    "values have {} dimensions, expected {}",
                    values.len(),
                    self.bias_vector.len()
                ),
            });
        }
        let result: Vec<f64> = values
            .iter()
            .zip(self.bias_vector.iter())
            .map(|(&v, &b)| {
                let biased = v * (1.0 + b * self.magnitude);
                biased.clamp(0.0, 1.0)
            })
            .collect();
        Ok(result)
    }

    /// Returns `true` if the bias is essentially neutral
    /// (magnitude below 0.05 and all vector components near zero).
    pub fn is_neutral(&self) -> bool {
        if self.magnitude < 0.05 {
            return true;
        }
        self.bias_vector.iter().all(|&b| b.abs() < 0.1)
    }

    /// Computes the net direction of the bias as the mean of the bias vector components.
    pub fn net_direction(&self) -> f64 {
        if self.bias_vector.is_empty() {
            return 0.0;
        }
        self.bias_vector.iter().sum::<f64>() / self.bias_vector.len() as f64
    }

    /// Returns the number of dimensions the bias operates on.
    pub fn dimensionality(&self) -> usize {
        self.bias_vector.len()
    }

    /// Adjusts the bias magnitude.
    ///
    /// # Errors
    /// Returns [`CognitionError::OutOfRange`] if the new magnitude is outside [0.0, 1.0].
    pub fn set_magnitude(&mut self, new_magnitude: f64) -> Result<(), CognitionError> {
        if !(0.0..=1.0).contains(&new_magnitude) {
            return Err(CognitionError::OutOfRange {
                field: "magnitude".to_string(),
                value: new_magnitude,
                min: 0.0,
                max: 1.0,
            });
        }
        self.magnitude = new_magnitude;
        Ok(())
    }

    /// Scales all bias vector components uniformly by a factor.
    ///
    /// # Errors
    /// Returns [`CognitionError::OutOfRange`] if the factor is outside [0.0, 1.0].
    pub fn scale(&mut self, factor: f64) -> Result<(), CognitionError> {
        if !(0.0..=1.0).contains(&factor) {
            return Err(CognitionError::OutOfRange {
                field: "scale_factor".to_string(),
                value: factor,
                min: 0.0,
                max: 1.0,
            });
        }
        for component in &mut self.bias_vector {
            *component *= factor;
        }
        Ok(())
    }

    /// Validates all field values are within valid ranges.
    pub fn validate(&self) -> Result<(), CognitionError> {
        if self.bias_vector.is_empty() {
            return Err(CognitionError::MissingInput(
                "bias_vector must not be empty".to_string(),
            ));
        }
        for (i, &component) in self.bias_vector.iter().enumerate() {
            if component < -1.0 || component > 1.0 {
                return Err(CognitionError::OutOfRange {
                    field: format!("bias_vector[{}]", i),
                    value: component,
                    min: -1.0,
                    max: 1.0,
                });
            }
        }
        if !(0.0..=1.0).contains(&self.magnitude) {
            return Err(CognitionError::OutOfRange {
                field: "magnitude".to_string(),
                value: self.magnitude,
                min: 0.0,
                max: 1.0,
            });
        }
        Ok(())
    }
}

impl fmt::Debug for PerspectiveBias {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PerspectiveBias")
            .field("bias_vector", &self.bias_vector)
            .field("magnitude", &self.magnitude)
            .finish()
    }
}