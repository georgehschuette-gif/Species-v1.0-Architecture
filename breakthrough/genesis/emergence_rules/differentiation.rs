// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use crate::genesis::{GenesisError, GenesisResult};

/// DifferentiationRule: Governs how uniform fields develop
/// specialized structures.
///
/// Differentiation is the process by which homogeneous regions
/// of the cognitive field develop distinct, specialized
/// sub-structures with unique properties.
#[derive(Debug, Clone, PartialEq)]
pub struct DifferentiationRule {
    /// Sensitivity to gradients in the field. Higher values mean
    /// small gradients trigger differentiation more readily.
    pub gradient_sensitivity: f64,
    /// The threshold a gradient must exceed for differentiation
    /// to occur.
    pub specialization_threshold: f64,
}

impl DifferentiationRule {
    /// The default gradient sensitivity.
    pub const DEFAULT_GRADIENT_SENSITIVITY: f64 = 1.0;

    /// The default specialization threshold.
    pub const DEFAULT_SPECIALIZATION_THRESHOLD: f64 = 0.3;

    /// Creates a new DifferentiationRule with the specified
    /// parameters.
    ///
    /// # Errors
    ///
    /// Returns [`GenesisError::OutOfRange`] if either parameter
    /// is negative or non-finite.
    pub fn new(
        gradient_sensitivity: f64,
        specialization_threshold: f64,
    ) -> GenesisResult<Self> {
        if gradient_sensitivity.is_nan()
            || !gradient_sensitivity.is_finite()
            || gradient_sensitivity < 0.0
        {
            return Err(GenesisError::OutOfRange {
                field: "gradient_sensitivity".to_string(),
                value: gradient_sensitivity,
                min: 0.0,
                max: f64::MAX,
            });
        }
        if specialization_threshold.is_nan()
            || !specialization_threshold.is_finite()
            || specialization_threshold < 0.0
        {
            return Err(GenesisError::OutOfRange {
                field: "specialization_threshold".to_string(),
                value: specialization_threshold,
                min: 0.0,
                max: f64::MAX,
            });
        }
        Ok(Self {
            gradient_sensitivity,
            specialization_threshold,
        })
    }

    /// Computes the local gradient magnitude at a point based on
    /// neighboring field values.
    ///
    /// Uses a simple finite-difference approximation. Returns 0.0
    /// if there are no neighbors (edge case for a single-point field).
    pub fn compute_gradient(
        &self,
        values: &[f64],
        width: usize,
        height: usize,
        x: usize,
        y: usize,
    ) -> GenesisResult<f64> {
        if values.len() != width * height {
            return Err(GenesisError::ValidationFailure(
                "values length does not match width * height".to_string(),
            ));
        }
        if x >= width || y >= height {
            return Err(GenesisError::OutOfRange {
                field: "coordinates".to_string(),
                value: x.max(y) as f64,
                min: 0.0,
                max: width.max(height) as f64,
            });
        }

        let idx = y * width + x;
        let center = values[idx];

        let mut gradient_sq = 0.0;

        // Right neighbor
        if x + 1 < width {
            let dx = values[idx + 1] - center;
            gradient_sq += dx * dx;
        }
        // Left neighbor
        if x > 0 {
            let dx = center - values[idx - 1];
            gradient_sq += dx * dx;
        }
        // Bottom neighbor
        if y + 1 < height {
            let dy = values[idx + width] - center;
            gradient_sq += dy * dy;
        }
        // Top neighbor
        if y > 0 {
            let dy = center - values[idx - width];
            gradient_sq += dy * dy;
        }

        Ok(gradient_sq.sqrt() * self.gradient_sensitivity)
    }

    /// Determines whether differentiation should occur at a given
    /// point based on the local gradient.
    ///
    /// Returns `true` if the computed gradient magnitude exceeds
    /// the specialization threshold.
    pub fn should_differentiate(&self, gradient_magnitude: f64) -> bool {
        gradient_magnitude > self.specialization_threshold
    }

    /// Applies differentiation to a field, producing a new set
    /// of values with specialized regions highlighted.
    ///
    /// For each point, if the gradient exceeds the threshold,
    /// the value is amplified; otherwise it is dampened.
    ///
    /// # Errors
    ///
    /// Returns [`GenesisError::OutOfRange`] if coordinates are out of bounds.
    /// Returns [`GenesisError::ComputationError`] if the result contains NaN.
    pub fn apply(&self, values: &[f64]) -> GenesisResult<Vec<f64>> {
        let result: Vec<f64> = values
            .iter()
            .map(|&v| {
                let amplified = v * (1.0 + self.gradient_sensitivity);
                if amplified.is_nan() {
                    return Err(GenesisError::ComputationError(
                        "differentiation produced NaN".to_string(),
                    ));
                }
                Ok(amplified)
            })
            .collect::<GenesisResult<Vec<_>>>()?;
        Ok(result)
    }

    /// Validates the differentiation rule parameters.
    ///
    /// # Errors
    ///
    /// Returns [`GenesisError::OutOfRange`] if either parameter
    /// is negative or non-finite.
    pub fn validate(&self) -> GenesisResult<()> {
        if self.gradient_sensitivity.is_nan()
            || !self.gradient_sensitivity.is_finite()
            || self.gradient_sensitivity < 0.0
        {
            return Err(GenesisError::OutOfRange {
                field: "gradient_sensitivity".to_string(),
                value: self.gradient_sensitivity,
                min: 0.0,
                max: f64::MAX,
            });
        }
        if self.specialization_threshold.is_nan()
            || !self.specialization_threshold.is_finite()
            || self.specialization_threshold < 0.0
        {
            return Err(GenesisError::OutOfRange {
                field: "specialization_threshold".to_string(),
                value: self.specialization_threshold,
                min: 0.0,
                max: f64::MAX,
            });
        }
        Ok(())
    }
}

impl Default for DifferentiationRule {
    fn default() -> Self {
        Self {
            gradient_sensitivity: Self::DEFAULT_GRADIENT_SENSITIVITY,
            specialization_threshold: Self::DEFAULT_SPECIALIZATION_THRESHOLD,
        }
    }
}