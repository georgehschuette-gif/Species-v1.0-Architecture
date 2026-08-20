// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.


/// ResonantCoupling: The strength of resonance between two entities.
///
/// Represents the coupling strength between two oscillating cognitive entities.
/// Strength is always in [0, 1], where 0 means no coupling and 1 means
/// perfect resonance.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ResonantCoupling {
    pub entity_a: u64,
    pub entity_b: u64,
    pub strength: f64,
}

impl ResonantCoupling {
    /// Creates a new resonant coupling between two entities.
    ///
    /// Strength is clamped to [0, 1].
    ///
    /// # Errors
    /// Returns `CouplingError::InvalidStrength` if strength is NaN.
    pub fn new(entity_a: u64, entity_b: u64, strength: f64) -> Result<Self, CouplingError> {
        if strength.is_nan() {
            return Err(CouplingError::InvalidStrength { strength });
        }
        Ok(Self {
            entity_a,
            entity_b,
            strength: strength.clamp(0.0, 1.0),
        })
    }

    /// Returns whether the coupling is active (non-zero strength).
    pub fn is_active(&self) -> bool {
        self.strength >= f64::EPSILON
    }

    /// Returns whether the coupling is at maximum strength.
    pub fn is_maximal(&self) -> bool {
        (self.strength - 1.0).abs() < f64::EPSILON
    }

    /// Returns the coupling strength as a percentage (0-100).
    pub fn strength_percent(&self) -> f64 {
        self.strength * 100.0
    }

    /// Scales the coupling strength by a multiplier.
    ///
    /// Returns a new coupling with the same entities but scaled strength.
    pub fn scale_strength(&self, factor: f64) -> Result<Self, CouplingError> {
        Self::new(self.entity_a, self.entity_b, self.strength * factor)
    }

    /// Returns the entity pair as a tuple.
    pub fn entities(&self) -> (u64, u64) {
        (self.entity_a, self.entity_b)
    }

    /// Checks whether this coupling involves the given entity.
    pub fn involves(&self, entity: u64) -> bool {
        self.entity_a == entity || self.entity_b == entity
    }

    /// Returns the reverse coupling (swapped entity order, same strength).
    pub fn reversed(&self) -> Self {
        Self {
            entity_a: self.entity_b,
            entity_b: self.entity_a,
            strength: self.strength,
        }
    }

    /// Computes the combined coupling from two couplings involving overlapping entities.
    pub fn combine(&self, other: &ResonantCoupling) -> Option<ResonantCoupling> {
        if self.entity_a == other.entity_a && self.entity_b == other.entity_b {
            let combined = (self.strength + other.strength) * 0.5;
            Some(ResonantCoupling {
                entity_a: self.entity_a,
                entity_b: self.entity_b,
                strength: combined.min(1.0),
            })
        } else if self.entity_a == other.entity_b && self.entity_b == other.entity_a {
            let combined = (self.strength + other.strength) * 0.5;
            Some(ResonantCoupling {
                entity_a: self.entity_a,
                entity_b: self.entity_b,
                strength: combined.min(1.0),
            })
        } else {
            None
        }
    }
}

impl Default for ResonantCoupling {
    fn default() -> Self {
        Self {
            entity_a: 0,
            entity_b: 0,
            strength: 0.0,
        }
    }
}

/// Error type for coupling validation failures.
#[derive(Debug, Clone, PartialEq)]
pub enum CouplingError {
    /// The strength value is NaN.
    InvalidStrength { strength: f64 },
}

impl std::fmt::Display for CouplingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CouplingError::InvalidStrength { strength } => {
                write!(f, "Invalid coupling strength {}: must be a valid number", strength)
            }
        }
    }
}

impl std::error::Error for CouplingError {}