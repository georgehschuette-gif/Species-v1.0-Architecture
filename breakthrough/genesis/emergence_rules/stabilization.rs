// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use crate::genesis::{GenesisError, GenesisResult};

/// StabilizationRule: Governs how dynamic systems reach and
/// maintain equilibrium.
///
/// The stabilization rule governs the convergence of the
/// cognitive ecosystem toward stable, self-sustaining states.
/// It applies damping to oscillations and detects when the
/// system has settled into equilibrium.
#[derive(Debug, Clone, PartialEq)]
pub struct StabilizationRule {
    /// Factor applied to reduce oscillations, in (0.0, 1.0].
    /// A value of 1.0 means no damping; lower values increase
    /// damping strength.
    pub damping_factor: f64,
    /// The variance threshold below which the system is
    /// considered stable.
    pub tolerance: f64,
}

impl StabilizationRule {
    /// The default damping factor.
    pub const DEFAULT_DAMPING_FACTOR: f64 = 0.5;

    /// The default tolerance for stability detection.
    pub const DEFAULT_TOLERANCE: f64 = 0.01;

    /// The minimum allowed damping factor.
    pub const MIN_DAMPING_FACTOR: f64 = 0.0;

    /// The maximum allowed damping factor.
    pub const MAX_DAMPING_FACTOR: f64 = 1.0;

    /// Creates a new StabilizationRule with the specified
    /// parameters.
    ///
    /// # Errors
    ///
    /// Returns [`GenesisError::OutOfRange`] if `damping_factor`
    /// is outside [0.0, 1.0] or `tolerance` is negative or
    /// non-finite.
    pub fn new(
        damping_factor: f64,
        tolerance: f64,
    ) -> GenesisResult<Self> {
        if damping_factor.is_nan() || !damping_factor.is_finite() {
            return Err(GenesisError::OutOfRange {
                field: "damping_factor".to_string(),
                value: damping_factor,
                min: Self::MIN_DAMPING_FACTOR,
                max: Self::MAX_DAMPING_FACTOR,
            });
        }
        let clamped_damping = damping_factor.clamp(
            Self::MIN_DAMPING_FACTOR,
            Self::MAX_DAMPING_FACTOR,
        );
        if tolerance.is_nan() || !tolerance.is_finite() {
            return Err(GenesisError::OutOfRange {
                field: "tolerance".to_string(),
                value: tolerance,
                min: 0.0,
                max: f64::MAX,
            });
        }
        if tolerance < 0.0 {
            return Err(GenesisError::OutOfRange {
                field: "tolerance".to_string(),
                value: tolerance,
                min: 0.0,
                max: f64::MAX,
            });
        }
        Ok(Self {
            damping_factor: clamped_damping,
            tolerance,
        })
    }

    /// Returns whether the system is considered stable based on
    /// the current variance.
    ///
    /// Stability requires variance to be below the tolerance
    /// threshold.
    pub fn is_stable(&self, variance: f64) -> bool {
        variance < self.tolerance
    }

    /// Applies damping to a value, reducing its magnitude by
    /// the damping factor.
    ///
    /// The damped value is computed as `value * damping_factor`.
    pub fn apply_damping(&self, value: f64) -> f64 {
        value * self.damping_factor
    }

    /// Runs a stabilization iteration on a set of values,
    /// applying damping and returning the new variance.
    ///
    /// Each value is damped and then the mean is subtracted
    /// to center the values around zero before computing
    /// the variance.
    ///
    /// # Errors
    ///
    /// Returns [`GenesisError::MissingInput`] if values is empty.
    /// Returns [`GenesisError::ComputationError`] if the input
    /// contains NaN or infinity.
    pub fn stabilize(
        &self,
        values: &[f64],
    ) -> GenesisResult<(Vec<f64>, f64)> {
        if values.is_empty() {
            return Err(GenesisError::MissingInput(
                "values cannot be empty".to_string(),
            ));
        }
        let damped: Vec<f64> = values
            .iter()
            .map(|&v| {
                if v.is_nan() || v.is_infinite() {
                    return Err(GenesisError::ComputationError(
                        "input values cannot be NaN or infinite".to_string(),
                    ));
                }
                Ok(v * self.damping_factor)
            })
            .collect::<GenesisResult<Vec<_>>>()?;

        let mean = damped.iter().sum::<f64>() / damped.len() as f64;
        let centered: Vec<f64> = damped.iter().map(|&v| v - mean).collect();
        let variance = centered
            .iter()
            .map(|&v| v * v)
            .sum::<f64>()
            / damped.len() as f64;

        Ok((centered, variance))
    }

    /// Runs multiple stabilization iterations until the system
    /// reaches equilibrium or the maximum number of iterations
    /// is exceeded.
    ///
    /// Returns the number of iterations performed and whether
    /// equilibrium was reached.
    ///
    /// # Errors
    ///
    /// Returns [`GenesisError::MissingInput`] if values is empty.
    /// Returns [`GenesisError::OutOfRange`] if `max_iterations` is zero.
    pub fn stabilize_until(
        &self,
        values: &[f64],
        max_iterations: usize,
    ) -> GenesisResult<(Vec<f64>, usize, bool)> {
        if max_iterations == 0 {
            return Err(GenesisError::OutOfRange {
                field: "max_iterations".to_string(),
                value: 0.0,
                min: 1.0,
                max: usize::MAX as f64,
            });
        }
        if values.is_empty() {
            return Err(GenesisError::MissingInput(
                "values cannot be empty".to_string(),
            ));
        }

        let mut current = values.to_vec();
        for i in 0..max_iterations {
            let (damped, variance) = self.stabilize(&current)?;
            current = damped;
            if self.is_stable(variance) {
                return Ok((current, i + 1, true));
            }
        }

        Ok((current, max_iterations, false))
    }

    /// Validates the stabilization rule parameters.
    ///
    /// # Errors
    ///
    /// Returns [`GenesisError::OutOfRange`] if damping_factor is
    /// outside [0.0, 1.0] or tolerance is negative or non-finite.
    pub fn validate(&self) -> GenesisResult<()> {
        if self.damping_factor.is_nan()
            || !self.damping_factor.is_finite()
            || self.damping_factor < Self::MIN_DAMPING_FACTOR
            || self.damping_factor > Self::MAX_DAMPING_FACTOR
        {
            return Err(GenesisError::OutOfRange {
                field: "damping_factor".to_string(),
                value: self.damping_factor,
                min: Self::MIN_DAMPING_FACTOR,
                max: Self::MAX_DAMPING_FACTOR,
            });
        }
        if self.tolerance.is_nan() || !self.tolerance.is_finite() {
            return Err(GenesisError::OutOfRange {
                field: "tolerance".to_string(),
                value: self.tolerance,
                min: 0.0,
                max: f64::MAX,
            });
        }
        Ok(())
    }
}

impl Default for StabilizationRule {
    fn default() -> Self {
        Self {
            damping_factor: Self::DEFAULT_DAMPING_FACTOR,
            tolerance: Self::DEFAULT_TOLERANCE,
        }
    }
}