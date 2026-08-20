// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.


/// GradientSmoothing: The reduction of spatial differences through diffusion.
///
/// Applies a smoothing kernel across a field to reduce
/// sharp gradients and noise in cognitive property distributions.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GradientSmoothing {
    pub smoothing_factor: f64,
}

impl GradientSmoothing {
    /// The minimum smoothing factor (no smoothing).
    pub const MIN_FACTOR: f64 = 0.0;
    /// The maximum smoothing factor (maximum smoothing).
    pub const MAX_FACTOR: f64 = 1.0;

    /// Creates a new gradient smoothing configuration.
    ///
    /// Smoothing factor is clamped to [0, 1].
    ///
    /// # Errors
    /// Returns `GradientError::InvalidFactor` if factor is NaN or outside [0, 1].
    pub fn new(smoothing_factor: f64) -> Result<Self, GradientError> {
        if smoothing_factor.is_nan()
            || smoothing_factor < Self::MIN_FACTOR
            || smoothing_factor > Self::MAX_FACTOR
        {
            return Err(GradientError::InvalidFactor { smoothing_factor });
        }
        Ok(Self { smoothing_factor })
    }

    /// Returns whether smoothing is active (non-zero factor).
    pub fn is_active(&self) -> bool {
        self.smoothing_factor > f64::EPSILON
    }

    /// Applies one smoothing step to a slice of values.
    ///
    /// Each value is moved toward the local average of itself and its neighbors.
    /// Boundary values are only averaged with their single neighbor.
    /// Returns the maximum change observed.
    pub fn smooth(&self, values: &mut [f64]) -> Result<f64, GradientError> {
        if values.is_empty() {
            return Err(GradientError::EmptySlice);
        }
        let factor = self.smoothing_factor;
        let old = values.to_vec();
        let len = old.len();
        let mut max_change = 0.0;

        for i in 0..len {
            let neighbor_avg = if len == 1 {
                old[i]
            } else if i == 0 {
                (old[i] + old[i + 1]) * 0.5
            } else if i == len - 1 {
                (old[i - 1] + old[i]) * 0.5
            } else {
                (old[i - 1] + old[i] + old[i + 1]) / 3.0
            };
            let new_val = old[i] + factor * (neighbor_avg - old[i]);
            let change = (new_val - old[i]).abs();
            if change > max_change {
                max_change = change;
            }
            values[i] = new_val;
        }

        Ok(max_change)
    }

    /// Applies multiple smoothing steps until convergence or max iterations.
    ///
    /// Returns the number of steps performed.
    pub fn smooth_until(&self, values: &mut [f64], max_steps: usize, tolerance: f64) -> usize {
        for step in 0..max_steps {
            match self.smooth(values) {
                Ok(max_change) => {
                    if max_change < tolerance {
                        return step + 1;
                    }
                }
                Err(_) => return step,
            }
        }
        max_steps
    }

    /// Scales the smoothing factor.
    pub fn scale_factor(&self, factor: f64) -> Result<Self, GradientError> {
        Self::new(self.smoothing_factor * factor)
    }
}

impl Default for GradientSmoothing {
    fn default() -> Self {
        Self {
            smoothing_factor: 0.3,
        }
    }
}

/// Error type for gradient smoothing failures.
#[derive(Debug, Clone, PartialEq)]
pub enum GradientError {
    /// The smoothing factor is NaN or outside [0, 1].
    InvalidFactor { smoothing_factor: f64 },
    /// The slice is empty.
    EmptySlice,
}

impl std::fmt::Display for GradientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GradientError::InvalidFactor { smoothing_factor } => {
                write!(f, "Invalid smoothing factor {}: must be in [0.0, 1.0]", smoothing_factor)
            }
            GradientError::EmptySlice => {
                write!(f, "Cannot smooth an empty slice")
            }
        }
    }
}

impl std::error::Error for GradientError {}