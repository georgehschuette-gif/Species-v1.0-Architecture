// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use crate::substrate::space::Distance;

/// AttractiveForce: The force drawing two entities together.
///
/// The force follows a linear falloff model: full magnitude at zero
/// distance, decreasing linearly to zero at the range boundary.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AttractiveForce {
    pub source: u64,
    pub target: u64,
    pub magnitude: f64,
    pub range: f64,
}

impl AttractiveForce {
    /// The minimum valid range.
    pub const MIN_RANGE: f64 = 0.0;

    /// Creates a new attractive force between two entities.
    ///
    /// # Errors
    /// Returns `ForceError::InvalidRange` if range is negative or NaN.
    /// Returns `ForceError::InvalidMagnitude` if magnitude is NaN.
    pub fn new(
        source: u64,
        target: u64,
        magnitude: f64,
        range: f64,
    ) -> Result<Self, ForceError> {
        if magnitude.is_nan() {
            return Err(ForceError::InvalidMagnitude { magnitude });
        }
        if range.is_nan() || range < Self::MIN_RANGE {
            return Err(ForceError::InvalidRange { range });
        }
        Ok(Self {
            source,
            target,
            magnitude,
            range,
        })
    }

    /// Returns the force strength at the given distance.
    ///
    /// Returns 0.0 if the distance exceeds the range.
    /// Returns 0.0 if the distance is NaN or negative.
    pub fn strength_at_distance(&self, distance: f64) -> f64 {
        if distance.is_nan() || distance < 0.0 || distance > self.range {
            return 0.0;
        }
        self.magnitude * (1.0 - distance / self.range)
    }

    /// Computes the force exerted on a target at a given position.
    pub fn force_at_position(
        &self,
        target_pos: &crate::substrate::space::Position,
        source_pos: &crate::substrate::space::Position,
    ) -> f64 {
        let dist = source_pos.distance_to(target_pos);
        self.strength_at_distance(dist.0)
    }

    /// Returns the total work this force can perform over its full range.
    pub fn total_work(&self) -> f64 {
        self.magnitude * self.range * 0.5
    }

    /// Checks whether the force is active (non-zero magnitude and range).
    pub fn is_active(&self) -> bool {
        self.magnitude.abs() > f64::EPSILON && self.range > f64::EPSILON
    }

    /// Scales the magnitude by a factor.
    pub fn scale_magnitude(&self, factor: f64) -> Result<Self, ForceError> {
        Self::new(
            self.source,
            self.target,
            self.magnitude * factor,
            self.range,
        )
    }

    /// Returns the source-target pair.
    pub fn pair(&self) -> (u64, u64) {
        (self.source, self.target)
    }

    /// Checks whether this force involves the given entity.
    pub fn involves(&self, entity: u64) -> bool {
        self.source == entity || self.target == entity
    }
}

impl Default for AttractiveForce {
    fn default() -> Self {
        Self {
            source: 0,
            target: 0,
            magnitude: 0.0,
            range: 0.0,
        }
    }
}

/// Error type for attractive force failures.
#[derive(Debug, Clone, PartialEq)]
pub enum ForceError {
    /// The magnitude is NaN.
    InvalidMagnitude { magnitude: f64 },
    /// The range is NaN or negative.
    InvalidRange { range: f64 },
}

impl std::fmt::Display for ForceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ForceError::InvalidMagnitude { magnitude } => {
                write!(f, "Invalid magnitude {}: must be a valid number", magnitude)
            }
            ForceError::InvalidRange { range } => {
                write!(f, "Invalid range {}: must be non-negative", range)
            }
        }
    }
}

impl std::error::Error for ForceError {}