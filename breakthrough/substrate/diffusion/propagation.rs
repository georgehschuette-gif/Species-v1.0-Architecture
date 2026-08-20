// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.


/// PropagationField: The field through which cognitive properties diffuse.
///
/// Models the spread of cognitive properties across a spatial grid
/// at a configurable diffusion rate. Values are updated iteratively
/// based on neighbor averaging weighted by the diffusion rate.
#[derive(Debug, Clone, PartialEq)]
pub struct PropagationField {
    pub values: Vec<f64>,
    pub diffusion_rate: f64,
}

impl PropagationField {
    /// The minimum valid diffusion rate.
    pub const MIN_RATE: f64 = 0.0;
    /// The maximum valid diffusion rate.
    pub const MAX_RATE: f64 = 1.0;

    /// Creates a new propagation field.
    ///
    /// All values are initialized to zero.
    ///
    /// # Errors
    /// Returns `DiffusionError::InvalidRate` if rate is outside [0, 1] or NaN.
    pub fn new(size: usize, diffusion_rate: f64) -> Result<Self, DiffusionError> {
        if diffusion_rate.is_nan() || diffusion_rate < Self::MIN_RATE || diffusion_rate > Self::MAX_RATE {
            return Err(DiffusionError::InvalidRate { rate: diffusion_rate });
        }
        if size == 0 {
            return Err(DiffusionError::EmptyField);
        }
        Ok(Self {
            values: vec![0.0; size],
            diffusion_rate,
        })
    }

    /// Returns the number of cells in the field.
    pub fn size(&self) -> usize {
        self.values.len()
    }

    /// Returns the value at the given index.
    pub fn get(&self, index: usize) -> f64 {
        self.values.get(index).copied().unwrap_or(0.0)
    }

    /// Sets the value at the given index.
    ///
    /// Silently ignores out-of-bounds indices.
    pub fn set(&mut self, index: usize, value: f64) -> Result<(), DiffusionError> {
        if value.is_nan() {
            return Err(DiffusionError::NaNValue);
        }
        if let Some(slot) = self.values.get_mut(index) {
            *slot = value;
        }
        Ok(())
    }

    /// Performs one diffusion step, averaging each cell with its neighbors.
    ///
    /// Cells at the boundaries lose their values proportionally to the diffusion rate.
    /// Returns the maximum change observed during this step.
    pub fn step(&mut self) -> f64 {
        let size = self.values.len();
        if size <= 1 {
            return 0.0;
        }
        let rate = self.diffusion_rate;
        let old = self.values.clone();
        let mut max_change = 0.0;

        for i in 0..size {
            let left = if i > 0 { old[i - 1] } else { old[i] };
            let right = if i < size - 1 { old[i + 1] } else { old[i] };
            let new_val = old[i] + rate * ((left + right) * 0.5 - old[i]);
            let change = (new_val - old[i]).abs();
            if change > max_change {
                max_change = change;
            }
            self.values[i] = new_val;
        }

        max_change
    }

    /// Performs multiple diffusion steps until convergence or max iterations.
    ///
    /// Returns the number of steps performed.
    pub fn simulate(&mut self, max_steps: usize, tolerance: f64) -> usize {
        for step in 0..max_steps {
            let max_change = self.step();
            if max_change < tolerance {
                return step + 1;
            }
        }
        max_steps
    }

    /// Adds a concentration pulse at the given index.
    pub fn inject(&mut self, index: usize, amount: f64) -> Result<(), DiffusionError> {
        if amount.is_nan() {
            return Err(DiffusionError::NaNValue);
        }
        let current = self.get(index);
        self.set(index, current + amount)
    }

    /// Returns the peak value in the field.
    pub fn peak_value(&self) -> f64 {
        self.values
            .iter()
            .copied()
            .fold(0.0, f64::max)
    }

    /// Returns the total "mass" across all cells.
    pub fn total_mass(&self) -> f64 {
        self.values.iter().sum()
    }

    /// Normalizes all values so the peak is 1.0.
    pub fn normalize(&mut self) {
        let peak = self.peak_value();
        if peak > f64::EPSILON {
            for v in &mut self.values {
                *v /= peak;
            }
        }
    }
}

impl Default for PropagationField {
    fn default() -> Self {
        Self {
            values: Vec::new(),
            diffusion_rate: 0.5,
        }
    }
}

/// Error type for propagation field failures.
#[derive(Debug, Clone, PartialEq)]
pub enum DiffusionError {
    /// The diffusion rate is invalid (outside [0, 1] or NaN).
    InvalidRate { rate: f64 },
    /// The field is empty (size is zero).
    EmptyField,
    /// The value is NaN.
    NaNValue,
}

impl std::fmt::Display for DiffusionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DiffusionError::InvalidRate { rate } => {
                write!(f, "Invalid diffusion rate {}: must be in [0.0, 1.0]", rate)
            }
            DiffusionError::EmptyField => {
                write!(f, "Cannot create an empty propagation field")
            }
            DiffusionError::NaNValue => {
                write!(f, "Field value cannot be NaN")
            }
        }
    }
}

impl std::error::Error for DiffusionError {}