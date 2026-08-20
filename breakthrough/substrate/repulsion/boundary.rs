// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.


/// CognitiveBoundary: A protective barrier maintained by repulsive forces.
///
/// Defines a circular region around an entity that other
/// entities must respect. The boundary has a radius and strength.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CognitiveBoundary {
    pub entity: u64,
    pub radius: f64,
    pub strength: f64,
}

impl CognitiveBoundary {
    /// The minimum valid radius.
    pub const MIN_RADIUS: f64 = 0.0;

    /// Creates a new cognitive boundary for an entity.
    ///
    /// # Errors
    /// Returns `BoundaryError::InvalidRadius` if radius is negative or NaN.
    /// Returns `BoundaryError::InvalidStrength` if strength is NaN or not in [0, 1].
    pub fn new(entity: u64, radius: f64, strength: f64) -> Result<Self, BoundaryError> {
        if radius.is_nan() || radius < Self::MIN_RADIUS {
            return Err(BoundaryError::InvalidRadius { radius });
        }
        if strength.is_nan() || strength < 0.0 || strength > 1.0 {
            return Err(BoundaryError::InvalidStrength { strength });
        }
        Ok(Self {
            entity,
            radius,
            strength,
        })
    }

    /// Returns whether the boundary is active (positive radius and strength).
    pub fn is_active(&self) -> bool {
        self.radius > f64::EPSILON && self.strength > f64::EPSILON
    }

    /// Checks whether a point at the given distance is inside the boundary.
    pub fn contains_distance(&self, distance: f64) -> bool {
        distance < self.radius && distance.is_finite()
    }

    /// Checks whether a position is inside the boundary relative to the entity's position.
    pub fn contains_position(
        &self,
        entity_pos: &crate::substrate::space::Position,
        other_pos: &crate::substrate::space::Position,
    ) -> bool {
        let dist = entity_pos.distance_to(other_pos);
        self.contains_distance(dist.0)
    }

    /// Returns the effective repulsion strength at the given distance from the boundary edge.
    pub fn repulsion_at(&self, distance_from_center: f64) -> f64 {
        if distance_from_center < self.radius {
            self.strength * (1.0 - distance_from_center / self.radius)
        } else {
            0.0
        }
    }

    /// Expands the boundary by the given amount.
    pub fn expand(&mut self, amount: f64) -> Result<(), BoundaryError> {
        if amount.is_nan() || amount < 0.0 {
            return Err(BoundaryError::InvalidRadius {
                radius: self.radius + amount,
            });
        }
        self.radius += amount;
        Ok(())
    }

    /// Contracts the boundary by the given amount, clamping at zero.
    pub fn contract(&mut self, amount: f64) -> Result<(), BoundaryError> {
        if amount.is_nan() || amount < 0.0 {
            return Err(BoundaryError::InvalidRadius { radius: amount });
        }
        self.radius = (self.radius - amount).max(Self::MIN_RADIUS);
        Ok(())
    }

    /// Scales the boundary strength by a factor.
    pub fn scale_strength(&mut self, factor: f64) -> Result<(), BoundaryError> {
        let new_strength = self.strength * factor;
        if new_strength.is_nan() || new_strength < 0.0 || new_strength > 1.0 {
            return Err(BoundaryError::InvalidStrength { strength: new_strength });
        }
        self.strength = new_strength;
        Ok(())
    }
}

impl Default for CognitiveBoundary {
    fn default() -> Self {
        Self {
            entity: 0,
            radius: 0.0,
            strength: 0.0,
        }
    }
}

/// Error type for boundary validation failures.
#[derive(Debug, Clone, PartialEq)]
pub enum BoundaryError {
    /// The radius is NaN or negative.
    InvalidRadius { radius: f64 },
    /// The strength is NaN or outside [0, 1].
    InvalidStrength { strength: f64 },
}

impl std::fmt::Display for BoundaryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BoundaryError::InvalidRadius { radius } => {
                write!(f, "Invalid radius {}: must be non-negative", radius)
            }
            BoundaryError::InvalidStrength { strength } => {
                write!(f, "Invalid strength {}: must be in [0.0, 1.0]", strength)
            }
        }
    }
}

impl std::error::Error for BoundaryError {}