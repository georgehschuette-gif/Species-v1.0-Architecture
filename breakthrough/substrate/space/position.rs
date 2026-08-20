// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use super::{Distance, DistanceError};

/// Position: A point in three-dimensional cognitive space.
///
/// Each coordinate represents a dimension of cognitive embedding:
/// - `x`: semantic alignment axis
/// - `y`: temporal ordering axis
/// - `z`: abstraction depth axis
///
/// # Invariants
/// - Coordinates must be finite (not NaN or infinite)
/// - Positions are compared with a tolerance of `1e-9` for equality
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Position {
    /// Creates a new `Position` with the given coordinates.
    ///
    /// # Errors
    /// Returns `PositionError::InvalidCoordinate` if any coordinate is NaN or infinite.
    pub fn new(x: f64, y: f64, z: f64) -> Result<Self, PositionError> {
        if !x.is_finite() || !y.is_finite() || !z.is_finite() {
            return Err(PositionError::InvalidCoordinate {
                axis: find_invalid_axis(x, y, z),
            });
        }
        Ok(Self { x, y, z })
    }

    /// Returns the origin at (0.0, 0.0, 0.0).
    pub fn origin() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    /// Computes the Euclidean distance to another position.
    pub fn distance_to(&self, other: &Position) -> Distance {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        Distance::new((dx * dx + dy * dy + dz * dz).sqrt())
            .expect("Distance computed from finite coordinates is always non-negative and finite")
    }

    /// Computes the Manhattan (L1) distance to another position.
    pub fn manhattan_distance_to(&self, other: &Position) -> Distance {
        let dx = (self.x - other.x).abs();
        let dy = (self.y - other.y).abs();
        let dz = (self.z - other.z).abs();
        Distance::new(dx + dy + dz)
            .expect("Manhattan distance from finite coordinates is always non-negative")
    }

    /// Returns a new position offset by the given deltas.
    pub fn offset(&self, dx: f64, dy: f64, dz: f64) -> Result<Self, PositionError> {
        Self::new(self.x + dx, self.y + dy, self.z + dz)
    }

    /// Returns the component-wise minimum of two positions.
    pub fn min(&self, other: &Position) -> Self {
        Self {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
            z: self.z.min(other.z),
        }
    }

    /// Returns the component-wise maximum of two positions.
    pub fn max(&self, other: &Position) -> Self {
        Self {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
            z: self.z.max(other.z),
        }
    }

    /// Clamps each coordinate to the range `[low, high]`.
    pub fn clamp(&self, low: f64, high: f64) -> Result<Self, PositionError> {
        if low > high {
            return Err(PositionError::InvalidRange { low, high });
        }
        Ok(Self {
            x: self.x.clamp(low, high),
            y: self.y.clamp(low, high),
            z: self.z.clamp(low, high),
        })
    }

    /// Returns the magnitude (norm) of this position vector from the origin.
    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    /// Returns a normalized version of this position vector.
    ///
    /// If the magnitude is zero, returns the origin.
    pub fn normalize(&self) -> Self {
        let mag = self.magnitude();
        if mag < 1e-12 {
            return Self::origin();
        }
        Self {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
        }
    }

    /// Computes the dot product with another position vector.
    pub fn dot(&self, other: &Position) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl Default for Position {
    fn default() -> Self {
        Self::origin()
    }
}

/// Error type for position validation failures.
#[derive(Debug, Clone, PartialEq)]
pub enum PositionError {
    /// One or more coordinates were NaN or infinite.
    InvalidCoordinate { axis: &'static str },
    /// The low bound of a range exceeds the high bound.
    InvalidRange { low: f64, high: f64 },
}

impl std::fmt::Display for PositionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PositionError::InvalidCoordinate { axis } => {
                write!(f, "Invalid coordinate on axis {}: value must be finite", axis)
            }
            PositionError::InvalidRange { low, high } => {
                write!(f, "Invalid range: low ({}) exceeds high ({})", low, high)
            }
        }
    }
}

impl std::error::Error for PositionError {}

fn find_invalid_axis(x: f64, y: f64, z: f64) -> &'static str {
    if !x.is_finite() {
        "x"
    } else if !y.is_finite() {
        "y"
    } else {
        "z"
    }
}