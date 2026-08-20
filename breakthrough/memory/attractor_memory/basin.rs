// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use crate::MemoryError;

use super::*;

/// AttractorBasin: A region of state space converging to a stable memory point.
///
/// Each basin represents a stored memory. The center encodes the memory content,
/// the radius defines the neighborhood of states that converge to that memory,
/// and the depth measures the robustness of the attractor against perturbation.
#[derive(Debug, Clone, PartialEq)]
pub struct AttractorBasin {
    /// Center coordinates representing the stored memory.
    pub center: Vec<f64>,
    /// Radius of the basin in state space.
    pub radius: f64,
    /// Depth of the attractor potential well in [0.0, 1.0].
    pub depth: f64,
    /// Number of times the basin has been visited.
    pub occupancy: usize,
    /// Unique identifier for this basin.
    pub basin_id: usize,
}

impl AttractorBasin {
    /// Minimum valid basin radius.
    pub const MIN_RADIUS: f64 = 0.0;
    /// Maximum valid basin radius.
    pub const MAX_RADIUS: f64 = 1.0;

    /// Creates a new AttractorBasin with the given center, radius, and depth.
    ///
    /// # Errors
    ///
    /// Returns [`MemoryError::DimensionMismatch`] if center is empty.
    /// Returns [`MemoryError::OutOfRange`] if radius or depth is outside [0.0, 1.0].
    pub fn new(center: Vec<f64>, radius: f64, depth: f64) -> Result<Self, MemoryError> {
        if center.is_empty() {
            return Err(MemoryError::DimensionMismatch {
                expected: 1,
                actual: 0,
            });
        }
        if !(Self::MIN_RADIUS..=Self::MAX_RADIUS).contains(&radius) {
            return Err(MemoryError::OutOfRange {
                field: "radius".into(),
                value: radius,
                min: Self::MIN_RADIUS,
                max: Self::MAX_RADIUS,
            });
        }
        if !(0.0..=1.0).contains(&depth) {
            return Err(MemoryError::OutOfRange {
                field: "depth".into(),
                value: depth,
                min: 0.0,
                max: 1.0,
            });
        }
        Ok(Self {
            center,
            radius,
            depth,
            occupancy: 0,
            basin_id: 0,
        })
    }

    /// Records a visit (enrollment) to this basin, increasing occupancy.
    pub fn enroll(&mut self, amount: f64) {
        self.occupancy += 1;
        self.depth = (self.depth + amount * 0.01).min(MAX_BASIN_DEPTH);
    }

    /// Dissipates the basin slightly, reducing depth and radius.
    pub fn dissipate(&mut self, amount: f64) {
        self.depth = (self.depth - amount).max(0.0);
        self.radius = (self.radius - amount * 0.1).max(0.0);
    }

    /// Returns whether a given position lies within this basin.
    pub fn contains(&self, position: &[f64]) -> bool {
        if position.len() != self.center.len() {
            return false;
        }
        self.distance_to_center(position) <= self.radius
    }

    /// Computes the Euclidean distance from the given position to the basin center.
    pub fn distance_to_center(&self, position: &[f64]) -> f64 {
        if position.len() != self.center.len() {
            return f64::INFINITY;
        }
        let sum_sq: f64 = position
            .iter()
            .zip(&self.center)
            .map(|(a, b)| (a - b).powi(2))
            .sum();
        sum_sq.sqrt()
    }

    /// Returns the occupancy ratio relative to a total visit budget.
    pub fn occupancy_ratio(&self, total_visits: usize) -> f64 {
        if total_visits == 0 {
            return 0.0;
        }
        self.occupancy as f64 / total_visits as f64
    }

    /// Returns the dimensionality of this basin.
    pub fn dimension(&self) -> usize {
        self.center.len()
    }

    /// Validates the basin state.
    pub fn validate(&self) -> Result<(), MemoryError> {
        Self::new(self.center.clone(), self.radius, self.depth)?;
        if self.occupancy > MAX_BASINS {
            return Err(MemoryError::CapacityExceeded {
                max: MAX_BASINS,
                attempted: self.occupancy,
            });
        }
        Ok(())
    }
}

