// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use crate::genesis::GenesisError;
use crate::genesis::GenesisResult;
use crate::genesis::ontology::{Entity, EntityState};

/// CognitiveField: The continuous medium in which entities exist and interact.
/// Analogous to a gravitational or electromagnetic field in physics.
///
/// The field is represented as a grid of values with configurable
/// dimensions and resolution. Field values encode the cognitive
/// potential at each point in the space.
#[derive(Debug, Clone, PartialEq)]
pub struct CognitiveField {
    /// The number of dimensions in the field grid.
    pub dimensions: usize,
    /// The resolution of the field, representing the spacing between
    /// grid points in abstract cognitive units.
    pub resolution: f64,
    /// The grid values stored in row-major order.
    pub values: Vec<f64>,
}

impl CognitiveField {
    /// The minimum allowed number of dimensions.
    pub const MIN_DIMENSIONS: usize = 1;

    /// The maximum allowed number of dimensions.
    pub const MAX_DIMENSIONS: usize = 128;

    /// The minimum valid resolution value.
    pub const MIN_RESOLUTION: f64 = 1e-12;

    /// Creates a new cognitive field with the given dimensions and
    /// resolution, initialized to zero.
    ///
    /// # Errors
    ///
    /// Returns [`GenesisError::OutOfRange`] if `dimensions` is zero
    /// or exceeds the maximum.
    /// Returns [`GenesisError::OutOfRange`] if `resolution` is not finite
    /// or is below the minimum.
    pub fn new(dimensions: usize, resolution: f64) -> GenesisResult<Self> {
        if dimensions < Self::MIN_DIMENSIONS {
            return Err(GenesisError::OutOfRange {
                field: "dimensions".to_string(),
                value: dimensions as f64,
                min: Self::MIN_DIMENSIONS as f64,
                max: Self::MAX_DIMENSIONS as f64,
            });
        }
        if dimensions > Self::MAX_DIMENSIONS {
            return Err(GenesisError::OutOfRange {
                field: "dimensions".to_string(),
                value: dimensions as f64,
                min: Self::MIN_DIMENSIONS as f64,
                max: Self::MAX_DIMENSIONS as f64,
            });
        }
        if !resolution.is_finite() || resolution < Self::MIN_RESOLUTION {
            return Err(GenesisError::OutOfRange {
                field: "resolution".to_string(),
                value: resolution,
                min: Self::MIN_RESOLUTION,
                max: f64::MAX,
            });
        }

        let size = dimensions * dimensions;
        Ok(Self {
            dimensions,
            resolution,
            values: vec![0.0; size],
        })
    }

    /// Returns the total number of grid points in the field.
    pub fn size(&self) -> usize {
        self.dimensions * self.dimensions
    }

    /// Samples the field value at the given coordinates.
    ///
    /// # Returns
    ///
    /// Returns the field value at `(x, y)` if coordinates are in bounds,
    /// or `None` if out of bounds.
    pub fn sample(&self, x: usize, y: usize) -> Option<f64> {
        if x >= self.dimensions || y >= self.dimensions {
            return None;
        }
        let idx = y * self.dimensions + x;
        self.values.get(idx).copied()
    }

    /// Sets the field value at the given coordinates.
    ///
    /// # Errors
    ///
    /// Returns [`GenesisError::OutOfRange`] if coordinates are out of bounds.
    pub fn set(&mut self, x: usize, y: usize, value: f64) -> GenesisResult<()> {
        if x >= self.dimensions || y >= self.dimensions {
            return Err(GenesisError::OutOfRange {
                field: "coordinates".to_string(),
                value: x.max(y) as f64,
                min: 0.0,
                max: self.dimensions as f64 - 1.0,
            });
        }
        if value.is_nan() {
            return Err(GenesisError::ComputationError(
                "field value cannot be NaN".to_string(),
            ));
        }
        let idx = y * self.dimensions + x;
        self.values[idx] = value;
        Ok(())
    }

    /// Resizes the field to new dimensions, preserving existing values
    /// where possible and zero-filling new positions.
    ///
    /// # Errors
    ///
    /// Returns [`GenesisError::OutOfRange`] if `new_dimensions` is zero
    /// or exceeds the maximum.
    pub fn resize(&mut self, new_dimensions: usize) -> GenesisResult<()> {
        if new_dimensions < Self::MIN_DIMENSIONS {
            return Err(GenesisError::OutOfRange {
                field: "new_dimensions".to_string(),
                value: new_dimensions as f64,
                min: Self::MIN_DIMENSIONS as f64,
                max: Self::MAX_DIMENSIONS as f64,
            });
        }
        if new_dimensions > Self::MAX_DIMENSIONS {
            return Err(GenesisError::OutOfRange {
                field: "new_dimensions".to_string(),
                value: new_dimensions as f64,
                min: Self::MIN_DIMENSIONS as f64,
                max: Self::MAX_DIMENSIONS as f64,
            });
        }

        let old_dim = self.dimensions;
        let mut new_values = vec![0.0; new_dimensions * new_dimensions];

        for y in 0..old_dim.min(new_dimensions) {
            for x in 0..old_dim.min(new_dimensions) {
                let old_idx = y * old_dim + x;
                let new_idx = y * new_dimensions + x;
                new_values[new_idx] = self.values[old_idx];
            }
        }

        self.dimensions = new_dimensions;
        self.values = new_values;
        Ok(())
    }

    /// Normalizes all field values so that the maximum absolute value
    /// becomes 1.0 (or all values remain zero if the field is uniform).
    pub fn normalize(&mut self) {
        let max_abs = self.values.iter().map(|v| v.abs()).fold(0.0, f64::max);
        if max_abs > 0.0 && max_abs.is_finite() {
            for v in &mut self.values {
                *v /= max_abs;
            }
        }
    }

    /// Returns the total energy of the field, computed as the sum of
    /// absolute values of all grid points.
    pub fn total_energy(&self) -> f64 {
        self.values.iter().map(|v| v.abs()).sum()
    }

    /// Returns the mean value across all grid points.
    pub fn mean(&self) -> f64 {
        if self.values.is_empty() {
            return 0.0;
        }
        self.values.iter().sum::<f64>() / self.values.len() as f64
    }

    /// Returns the variance of field values across all grid points.
    pub fn variance(&self) -> f64 {
        if self.values.is_empty() {
            return 0.0;
        }
        let mean = self.mean();
        self.values
            .iter()
            .map(|v| (v - mean).powi(2))
            .sum::<f64>()
            / self.values.len() as f64
    }

    /// Clears all field values to zero.
    pub fn clear(&mut self) {
        for v in &mut self.values {
            *v = 0.0;
        }
    }

    /// Applies a scaling factor to all field values.
    ///
    /// # Errors
    ///
    /// Returns [`GenesisError::ComputationError`] if `factor` is NaN or infinite.
    pub fn scale(&mut self, factor: f64) -> GenesisResult<()> {
        if factor.is_nan() || factor.is_infinite() {
            return Err(GenesisError::ComputationError(
                "scale factor must be finite".to_string(),
            ));
        }
        for v in &mut self.values {
            *v *= factor;
        }
        Ok(())
    }

    /// Validates the field, ensuring dimensions are in valid range,
    /// resolution is positive and finite, and the values vector has
    /// the correct length.
    ///
    /// # Errors
    ///
    /// Returns [`GenesisError::OutOfRange`] if dimensions are out of bounds.
    /// Returns [`GenesisError::OutOfRange`] if resolution is invalid.
    /// Returns [`GenesisError::ValidationFailure`] if values length does not
    /// match dimensions squared.
    pub fn validate(&self) -> GenesisResult<()> {
        if self.dimensions < Self::MIN_DIMENSIONS || self.dimensions > Self::MAX_DIMENSIONS {
            return Err(GenesisError::OutOfRange {
                field: "dimensions".to_string(),
                value: self.dimensions as f64,
                min: Self::MIN_DIMENSIONS as f64,
                max: Self::MAX_DIMENSIONS as f64,
            });
        }
        if !self.resolution.is_finite() || self.resolution < Self::MIN_RESOLUTION {
            return Err(GenesisError::OutOfRange {
                field: "resolution".to_string(),
                value: self.resolution,
                min: Self::MIN_RESOLUTION,
                max: f64::MAX,
            });
        }
        let expected_size = self.dimensions * self.dimensions;
        if self.values.len() != expected_size {
            return Err(GenesisError::ValidationFailure(format!(
                "values length {} does not match dimensions {} x {} = {}",
                self.values.len(),
                self.dimensions,
                self.dimensions,
                expected_size
            )));
        }
        for (i, v) in self.values.iter().enumerate() {
            if v.is_nan() {
                return Err(GenesisError::ComputationError(format!(
                    "field value at index {} is NaN",
                    i
                )));
            }
        }
        Ok(())
    }

    /// Checks if this field is compatible with another field for
    /// operations that require matching dimensions.
    ///
    /// Returns `true` if both fields have the same dimensions and
    /// resolution.
    pub fn is_compatible_with(&self, other: &Self) -> bool {
        self.dimensions == other.dimensions
            && (self.resolution - other.resolution).abs() < f64::EPSILON
    }

    /// Extracts a slice of entity references whose energy exceeds
    /// the activation threshold in the context of this field.
    ///
    /// Returns an empty slice if no entities meet the threshold.
    pub fn active_entities<'a>(
        &self,
        entities: &'a [Entity],
        threshold: f64,
    ) -> Vec<&'a Entity> {
        entities
            .iter()
            .filter(|e| e.energy >= threshold && e.state == EntityState::Active)
            .collect()
    }
}

impl Default for CognitiveField {
    fn default() -> Self {
        Self {
            dimensions: 2,
            resolution: 1.0,
            values: vec![0.0; 4],
        }
    }
}