// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.


/// CognitiveBond: A persistent connection formed through attraction.
///
/// Bonds have a strength that can degrade over time and are
/// tracked with their formation time for age-based analysis.
#[derive(Debug, Clone, PartialEq)]
pub struct CognitiveBond {
    pub entity_a: u64,
    pub entity_b: u64,
    pub strength: f64,
    pub formation_time: f64,
}

impl CognitiveBond {
    /// The minimum bond strength.
    pub const MIN_STRENGTH: f64 = 0.0;
    /// The maximum bond strength.
    pub const MAX_STRENGTH: f64 = 1.0;

    /// Creates a new cognitive bond between two entities.
    ///
    /// Strength is clamped to [0, 1].
    ///
    /// # Errors
    /// Returns `BondError::InvalidStrength` if strength is NaN.
    pub fn new(
        entity_a: u64,
        entity_b: u64,
        strength: f64,
        time: f64,
    ) -> Result<Self, BondError> {
        if strength.is_nan() {
            return Err(BondError::InvalidStrength { strength });
        }
        Ok(Self {
            entity_a,
            entity_b,
            strength: strength.clamp(Self::MIN_STRENGTH, Self::MAX_STRENGTH),
            formation_time: time,
        })
    }

    /// Returns whether the bond is active (positive strength).
    pub fn is_active(&self) -> bool {
        self.strength >= f64::EPSILON
    }

    /// Returns whether this bond connects the given entity.
    pub fn involves(&self, entity: u64) -> bool {
        self.entity_a == entity || self.entity_b == entity
    }

    /// Ages the bond by the given time delta, potentially weakening it.
    ///
    /// The bond strength decreases linearly with age at a rate of
    /// `decay_rate * delta`. Strength is clamped at zero.
    pub fn age(&mut self, delta: f64, decay_rate: f64) -> Result<(), BondError> {
        if delta.is_nan() || delta < 0.0 {
            return Err(BondError::InvalidDelta { delta });
        }
        if decay_rate.is_nan() || decay_rate < 0.0 {
            return Err(BondError::InvalidDecayRate { decay_rate });
        }
        self.strength = (self.strength - decay_rate * delta).max(Self::MIN_STRENGTH);
        Ok(())
    }

    /// Strengthens the bond by the given amount.
    ///
    /// Returns an error if the amount is NaN.
    pub fn strengthen(&mut self, amount: f64) -> Result<(), BondError> {
        if amount.is_nan() {
            return Err(BondError::InvalidStrength { strength: amount });
        }
        self.strength = (self.strength + amount).min(Self::MAX_STRENGTH);
        Ok(())
    }

    /// Returns the age of this bond at a given current time.
    pub fn age_at(&self, current_time: f64) -> f64 {
        (current_time - self.formation_time).max(0.0)
    }

    /// Checks whether this bond is older than the specified threshold.
    pub fn is_older_than(&self, threshold: f64, current_time: f64) -> bool {
        self.age_at(current_time) > threshold
    }

    /// Returns the entity pair as a tuple.
    pub fn pair(&self) -> (u64, u64) {
        (self.entity_a, self.entity_b)
    }

    /// Computes the effective bond strength after accounting for age-related decay.
    pub fn effective_strength(&self, current_time: f64, decay_rate: f64) -> f64 {
        let age = self.age_at(current_time);
        self.strength * (-decay_rate * age).exp()
    }

    /// Checks whether this bond is a self-loop.
    pub fn is_self_loop(&self) -> bool {
        self.entity_a == self.entity_b
    }
}

impl Default for CognitiveBond {
    fn default() -> Self {
        Self {
            entity_a: 0,
            entity_b: 0,
            strength: 0.0,
            formation_time: 0.0,
        }
    }
}

/// Error type for bond operation failures.
#[derive(Debug, Clone, PartialEq)]
pub enum BondError {
    /// The strength value is NaN.
    InvalidStrength { strength: f64 },
    /// The time delta is NaN or negative.
    InvalidDelta { delta: f64 },
    /// The decay rate is NaN or negative.
    InvalidDecayRate { decay_rate: f64 },
}

impl std::fmt::Display for BondError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BondError::InvalidStrength { strength } => {
                write!(f, "Invalid bond strength {}: must be a valid number", strength)
            }
            BondError::InvalidDelta { delta } => {
                write!(f, "Invalid delta {}: must be non-negative", delta)
            }
            BondError::InvalidDecayRate { decay_rate } => {
                write!(f, "Invalid decay rate {}: must be non-negative", decay_rate)
            }
        }
    }
}

impl std::error::Error for BondError {}