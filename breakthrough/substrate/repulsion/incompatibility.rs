// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.


/// IncompatibilityField: The spatial field representing repulsion between entities.
///
/// Stores per-cell incompatibility values that determine how strongly
/// entities at corresponding positions are pushed apart.
/// Values are guaranteed to be in [0, 1].
#[derive(Debug, Clone, PartialEq)]
pub struct IncompatibilityField {
    pub values: Vec<f64>,
    pub resolution: f64,
}

impl IncompatibilityField {
    /// The minimum valid resolution.
    pub const MIN_RESOLUTION: f64 = 0.0;
    /// The maximum incompatibility value.
    pub const MAX_INCOMPATIBILITY: f64 = 1.0;

    /// Creates a new incompatibility field with the given size and resolution.
    ///
    /// All cells are initialized to zero incompatibility.
    ///
    /// # Errors
    /// Returns `IncompatibilityError::ZeroResolution` if resolution is zero.
    pub fn new(size: usize, resolution: f64) -> Result<Self, IncompatibilityError> {
        if resolution <= Self::MIN_RESOLUTION {
            return Err(IncompatibilityError::ZeroResolution);
        }
        Ok(Self {
            values: vec![0.0; size],
            resolution,
        })
    }

    /// Returns the number of cells in the field.
    pub fn size(&self) -> usize {
        self.values.len()
    }

    /// Returns the incompatibility value at the given index.
    ///
    /// Returns 0.0 for out-of-bounds indices.
    pub fn get(&self, index: usize) -> f64 {
        self.values.get(index).copied().unwrap_or(0.0)
    }

    /// Sets the incompatibility value at the given index.
    ///
    /// Silently ignores out-of-bounds indices. Values are clamped to [0, 1].
    pub fn set(&mut self, index: usize, value: f64) -> Result<(), IncompatibilityError> {
        if value.is_nan() || value < Self::MIN_RESOLUTION || value > Self::MAX_INCOMPATIBILITY {
            return Err(IncompatibilityError::InvalidValue { value });
        }
        if let Some(slot) = self.values.get_mut(index) {
            *slot = value;
        }
        Ok(())
    }

    /// Returns the maximum incompatibility value in the field.
    pub fn max_value(&self) -> f64 {
        self.values
            .iter()
            .copied()
            .fold(0.0, f64::max)
    }

    /// Returns the average incompatibility across all cells.
    pub fn average(&self) -> f64 {
        if self.values.is_empty() {
            0.0
        } else {
            self.values.iter().sum::<f64>() / self.values.len() as f64
        }
    }

    /// Returns the index of the cell with the highest incompatibility.
    pub fn peak_index(&self) -> Option<usize> {
        self.values
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(i, _)| i)
    }

    /// Normalizes all values so the maximum incompatibility is 1.0.
    /// Does nothing if the maximum is zero.
    pub fn normalize(&mut self) {
        let max = self.max_value();
        if max > f64::EPSILON {
            for v in &mut self.values {
                *v /= max;
            }
        }
    }

    /// Computes the incompatibility between two indices.
    pub fn incompatibility_between(&self, a: usize, b: usize) -> f64 {
        let va = self.get(a);
        let vb = self.get(b);
        (va + vb) * 0.5
    }

    /// Applies a repulsion pulse at the given index.
    pub fn pulse(&mut self, index: usize, magnitude: f64) -> Result<(), IncompatibilityError> {
        if magnitude.is_nan() {
            return Err(IncompatibilityError::InvalidValue { value: magnitude });
        }
        let current = self.get(index);
        self.set(index, current + magnitude)
    }

    /// Decays all values by a given factor (multiplicative).
    pub fn decay_all(&mut self, factor: f64) {
        let decay = factor.clamp(0.0, 1.0);
        for v in &mut self.values {
            *v *= decay;
        }
    }
}

impl Default for IncompatibilityField {
    fn default() -> Self {
        Self {
            values: Vec::new(),
            resolution: 1.0,
        }
    }
}

/// Error type for incompatibility field failures.
#[derive(Debug, Clone, PartialEq)]
pub enum IncompatibilityError {
    /// The resolution is zero or negative.
    ZeroResolution,
    /// The value is NaN or outside [0, 1].
    InvalidValue { value: f64 },
}

impl std::fmt::Display for IncompatibilityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IncompatibilityError::ZeroResolution => write!(f, "Resolution must be positive"),
            IncompatibilityError::InvalidValue { value } => {
                write!(f, "Invalid incompatibility value {}: must be in [0.0, 1.0]", value)
            }
        }
    }
}

impl std::error::Error for IncompatibilityError {}