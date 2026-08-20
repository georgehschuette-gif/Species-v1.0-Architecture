// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.


/// CognitiveMixing: The blending of cognitive properties across entities.
///
/// Controls how strongly two entities influence each other's
/// properties. The blend factor determines the weight given
/// to the other entity's state.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CognitiveMixing {
    pub blend_factor: f64,
}

impl CognitiveMixing {
    /// The minimum blend factor (no mixing).
    pub const MIN_FACTOR: f64 = 0.0;
    /// The maximum blend factor (full absorption).
    pub const MAX_FACTOR: f64 = 1.0;

    /// Creates a new cognitive mixing configuration.
    ///
    /// Blend factor is clamped to [0, 1].
    ///
    /// # Errors
    /// Returns `MixingError::InvalidFactor` if factor is NaN or outside [0, 1].
    pub fn new(blend_factor: f64) -> Result<Self, MixingError> {
        if blend_factor.is_nan() || blend_factor < Self::MIN_FACTOR || blend_factor > Self::MAX_FACTOR {
            return Err(MixingError::InvalidFactor { blend_factor });
        }
        Ok(Self { blend_factor })
    }

    /// Returns whether mixing is active (non-zero blend factor).
    pub fn is_active(&self) -> bool {
        self.blend_factor > f64::EPSILON
    }

    /// Returns whether mixing is at maximum (full absorption).
    pub fn is_maximal(&self) -> bool {
        (self.blend_factor - 1.0).abs() < f64::EPSILON
    }

    /// Blends two values using this mixing configuration.
    ///
    /// The result is `a * (1 - factor) + b * factor`.
    pub fn blend(&self, a: f64, b: f64) -> f64 {
        a * (1.0 - self.blend_factor) + b * self.blend_factor
    }

    /// Blends two slices element-wise, writing the result into `output`.
    ///
    /// Returns an error if the slices have different lengths or are empty.
    pub fn blend_slices(
        &self,
        a: &[f64],
        b: &[f64],
        output: &mut [f64],
    ) -> Result<(), MixingError> {
        if a.len() != b.len() || a.len() != output.len() {
            return Err(MixingError::LengthMismatch {
                a_len: a.len(),
                b_len: b.len(),
                output_len: output.len(),
            });
        }
        if a.is_empty() {
            return Err(MixingError::EmptySlice);
        }
        for ((va, vb), out) in a.iter().zip(b.iter()).zip(output.iter_mut()) {
            *out = self.blend(*va, *vb);
        }
        Ok(())
    }

    /// Scales the blend factor.
    pub fn scale_factor(&self, factor: f64) -> Result<Self, MixingError> {
        Self::new(self.blend_factor * factor)
    }

    /// Returns the inverse mixing (the original entity retains more).
    pub fn inverse(&self) -> Result<Self, MixingError> {
        Self::new(1.0 - self.blend_factor)
    }
}

impl Default for CognitiveMixing {
    fn default() -> Self {
        Self {
            blend_factor: 0.5,
        }
    }
}

/// Error type for mixing failures.
#[derive(Debug, Clone, PartialEq)]
pub enum MixingError {
    /// The blend factor is NaN or outside [0, 1].
    InvalidFactor { blend_factor: f64 },
    /// Slice lengths do not match.
    LengthMismatch { a_len: usize, b_len: usize, output_len: usize },
    /// The slice is empty.
    EmptySlice,
}

impl std::fmt::Display for MixingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MixingError::InvalidFactor { blend_factor } => {
                write!(f, "Invalid blend factor {}: must be in [0.0, 1.0]", blend_factor)
            }
            MixingError::LengthMismatch {
                a_len,
                b_len,
                output_len,
            } => {
                write!(
                    f,
                    "Slice length mismatch: a={}, b={}, output={}",
                    a_len, b_len, output_len
                )
            }
            MixingError::EmptySlice => {
                write!(f, "Cannot blend empty slices")
            }
        }
    }
}

impl std::error::Error for MixingError {}