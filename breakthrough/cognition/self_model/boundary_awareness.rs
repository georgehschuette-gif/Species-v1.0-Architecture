// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use super::*;

/// BoundaryAwareness: The system's knowledge of its own limits.
pub struct BoundaryAwareness {
    pub known_limits: Vec<f64>,
    pub uncertainty_radius: f64,
}

impl BoundaryAwareness {
    /// Create a new boundary awareness module with known limits and uncertainty radius.
    pub fn new(known_limits: Vec<f64>, uncertainty_radius: f64) -> Result<Self, CognitionError> {
        if known_limits.is_empty() {
            return Err(CognitionError::MissingInput(
                "known_limits must not be empty".into(),
            ));
        }
        for (i, &limit) in known_limits.iter().enumerate() {
            if limit < 0.0 {
                return Err(CognitionError::OutOfRange {
                    field: format!("known_limits[{}]", i),
                    value: limit,
                    min: 0.0,
                    max: f64::INFINITY,
                });
            }
        }
        if uncertainty_radius < 0.0 {
            return Err(CognitionError::OutOfRange {
                field: "uncertainty_radius".to_string(),
                value: uncertainty_radius,
                min: 0.0,
                max: f64::INFINITY,
            });
        }
        Ok(Self {
            known_limits,
            uncertainty_radius,
        })
    }

    /// Check if a value is within known boundaries given the uncertainty radius.
    pub fn is_within_bounds(&self, value: f64) -> bool {
        value >= -self.uncertainty_radius && value <= self.known_limits[0] + self.uncertainty_radius
    }

    /// Expand the known limits to accommodate a new observed value.
    pub fn expand_if_necessary(&mut self, value: f64) -> bool {
        let upper_bound = self.known_limits[0];
        if value > upper_bound {
            self.known_limits[0] = value;
            true
        } else {
            false
        }
    }

    /// Return the distance from a value to the nearest boundary.
    pub fn distance_to_boundary(&self, value: f64) -> f64 {
        let upper = self.known_limits[0];
        let lower = 0.0;
        let dist_to_upper = (value - upper).abs();
        let dist_to_lower = (value - lower).abs();
        dist_to_upper.min(dist_to_lower)
    }

    /// Compute a safety margin: how far a value is from the boundary, accounting for uncertainty.
    pub fn safety_margin(&self, value: f64) -> f64 {
        let upper = self.known_limits[0] - self.uncertainty_radius;
        let lower = self.uncertainty_radius;
        if value < lower {
            value - lower
        } else if value > upper {
            upper - value
        } else {
            (upper - value).min(value - lower)
        }
    }

    /// Add a new known limit dimension.
    pub fn add_limit(&mut self, limit: f64) -> Result<(), CognitionError> {
        if limit < 0.0 {
            return Err(CognitionError::OutOfRange {
                field: "limit".to_string(),
                value: limit,
                min: 0.0,
                max: f64::INFINITY,
            });
        }
        self.known_limits.push(limit);
        Ok(())
    }

    /// Remove a known limit at the given index.
    pub fn remove_limit(&mut self, index: usize) -> Result<(), CognitionError> {
        if index >= self.known_limits.len() {
            return Err(CognitionError::OutOfRange {
                field: "index".to_string(),
                value: index as f64,
                min: 0.0,
                max: (self.known_limits.len() as f64).max(1.0) - 1.0,
            });
        }
        self.known_limits.remove(index);
        if self.known_limits.is_empty() {
            return Err(CognitionError::InvalidState(
                "cannot remove last known limit".into(),
            ));
        }
        Ok(())
    }

    /// Update the uncertainty radius.
    pub fn set_uncertainty_radius(&mut self, radius: f64) -> Result<(), CognitionError> {
        if radius < 0.0 {
            return Err(CognitionError::OutOfRange {
                field: "uncertainty_radius".to_string(),
                value: radius,
                min: 0.0,
                max: f64::INFINITY,
            });
        }
        self.uncertainty_radius = radius;
        Ok(())
    }

    /// Return the number of known limit dimensions.
    pub fn dimension_count(&self) -> usize {
        self.known_limits.len()
    }

    /// Return a reference to the known limits.
    pub fn limits(&self) -> &[f64] {
        &self.known_limits
    }

    /// Compute the average known limit across all dimensions.
    pub fn average_limit(&self) -> f64 {
        if self.known_limits.is_empty() {
            return 0.0;
        }
        self.known_limits.iter().sum::<f64>() / self.known_limits.len() as f64
    }

    /// Validate the boundary awareness configuration.
    pub fn validate(&self) -> Result<(), CognitionError> {
        if self.known_limits.is_empty() {
            return Err(CognitionError::MissingInput(
                "known_limits must not be empty".into(),
            ));
        }
        for (i, &limit) in self.known_limits.iter().enumerate() {
            if limit < 0.0 {
                return Err(CognitionError::OutOfRange {
                    field: format!("known_limits[{}]", i),
                    value: limit,
                    min: 0.0,
                    max: f64::INFINITY,
                });
            }
        }
        if self.uncertainty_radius < 0.0 {
            return Err(CognitionError::OutOfRange {
                field: "uncertainty_radius".to_string(),
                value: self.uncertainty_radius,
                min: 0.0,
                max: f64::INFINITY,
            });
        }
        Ok(())
    }

    /// Reset all known limits to zero and uncertainty radius to zero.
    pub fn reset(&mut self) {
        for limit in &mut self.known_limits {
            *limit = 0.0;
        }
        self.uncertainty_radius = 0.0;
    }
}

impl fmt::Debug for BoundaryAwareness {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BoundaryAwareness")
            .field("known_limits", &self.known_limits)
            .field("uncertainty_radius", &self.uncertainty_radius)
            .finish()
    }
}