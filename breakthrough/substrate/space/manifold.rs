// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use crate::substrate::space::{Distance, Position};

/// SpatialManifold: The continuous space in which cognitive entities reside.
///
/// Defines the dimensionality and bounding region of the cognitive embedding space.
/// All positions within a manifold must lie within its bounds.
#[derive(Debug, Clone, PartialEq)]
pub struct SpatialManifold {
    pub dimension: usize,
    pub bounds: (Position, Position),
}

impl SpatialManifold {
    /// The minimum allowed dimension for a spatial manifold.
    pub const MIN_DIMENSION: usize = 1;
    /// The maximum allowed dimension for a spatial manifold.
    pub const MAX_DIMENSION: usize = 1024;

    /// Creates a new `SpatialManifold` with the given dimensionality and bounds.
    ///
    /// # Errors
    /// Returns `ManifoldError::InvalidDimension` if dimension is zero or exceeds `MAX_DIMENSION`.
    /// Returns `ManifoldError::InvalidBounds` if any bound coordinate is invalid
    /// or if any component of `min` exceeds the corresponding component of `max`.
    pub fn new(
        dimension: usize,
        min: Position,
        max: Position,
    ) -> Result<Self, ManifoldError> {
        if dimension < Self::MIN_DIMENSION || dimension > Self::MAX_DIMENSION {
            return Err(ManifoldError::InvalidDimension { dimension });
        }
        if min.x > max.x || min.y > max.y || min.z > max.z {
            return Err(ManifoldError::InvalidBounds { min, max });
        }
        Ok(Self {
            dimension,
            bounds: (min, max),
        })
    }

    /// Returns whether a position lies within the manifold's bounds.
    pub fn contains(&self, position: &Position) -> bool {
        position.x >= self.bounds.0.x
            && position.x <= self.bounds.1.x
            && position.y >= self.bounds.0.y
            && position.y <= self.bounds.1.y
            && position.z >= self.bounds.0.z
            && position.z <= self.bounds.1.z
    }

    /// Clamps a position to lie within the manifold's bounds.
    pub fn clamp(&self, position: &Position) -> Position {
        Position {
            x: position.x.clamp(self.bounds.0.x, self.bounds.1.x),
            y: position.y.clamp(self.bounds.0.y, self.bounds.1.y),
            z: position.z.clamp(self.bounds.0.z, self.bounds.1.z),
        }
    }

    /// Returns the size (edge lengths) of the bounding box.
    pub fn size(&self) -> Position {
        Position {
            x: self.bounds.1.x - self.bounds.0.x,
            y: self.bounds.1.y - self.bounds.0.y,
            z: self.bounds.1.z - self.bounds.0.z,
        }
    }

    /// Returns the volume of the bounding box.
    pub fn volume(&self) -> f64 {
        let size = self.size();
        size.x * size.y * size.z
    }

    /// Returns the center point of the manifold.
    pub fn center(&self) -> Position {
        Position {
            x: (self.bounds.0.x + self.bounds.1.x) * 0.5,
            y: (self.bounds.0.y + self.bounds.1.y) * 0.5,
            z: (self.bounds.0.z + self.bounds.1.z) * 0.5,
        }
    }

    /// Computes the diagonal distance of the bounding box.
    pub fn diagonal(&self) -> Distance {
        self.bounds.0.distance_to(&self.bounds.1)
    }

    /// Computes the distance between two positions, both clamped to this manifold.
    pub fn bounded_distance(&self, a: &Position, b: &Position) -> Distance {
        let a_clamped = self.clamp(a);
        let b_clamped = self.clamp(b);
        a_clamped.distance_to(&b_clamped)
    }
}

impl Default for SpatialManifold {
    fn default() -> Self {
        Self {
            dimension: 3,
            bounds: (Position::origin(), Position::new(100.0, 100.0, 100.0).unwrap()),
        }
    }
}

/// Error type for manifold validation failures.
#[derive(Debug, Clone, PartialEq)]
pub enum ManifoldError {
    /// The dimension is zero or exceeds the maximum allowed.
    InvalidDimension { dimension: usize },
    /// The minimum bound exceeds the maximum bound on one or more axes.
    InvalidBounds { min: Position, max: Position },
}

impl std::fmt::Display for ManifoldError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ManifoldError::InvalidDimension { dimension } => {
                write!(
                    f,
                    "Invalid dimension {}: must be between 1 and {}",
                    dimension,
                    SpatialManifold::MAX_DIMENSION
                )
            }
            ManifoldError::InvalidBounds { min, max } => {
                write!(
                    f,
                    "Invalid bounds: min ({:?}) exceeds max ({:?}) on one or more axes",
                    min, max
                )
            }
        }
    }
}

impl std::error::Error for ManifoldError {}