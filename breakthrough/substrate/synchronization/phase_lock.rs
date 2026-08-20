// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.


/// PhaseLock: The synchronization of oscillatory phases between entities.
///
/// Tracks the phase difference between two entities and
/// whether they have achieved phase lock (stable
/// synchronization).
#[derive(Debug, Clone, PartialEq)]
pub struct PhaseLock {
    pub entity_a: u64,
    pub entity_b: u64,
    pub phase_difference: f64,
    pub locked: bool,
}

impl PhaseLock {
    /// Full circle in radians.
    pub const TAU: f64 = 2.0 * std::f64::consts::PI;
    /// The maximum phase difference for lock acquisition.
    pub const LOCK_THRESHOLD: f64 = 0.1;

    /// Creates a new phase lock between two entities.
    ///
    /// Phase difference is normalized to [0, 2π).
    ///
    /// # Errors
    /// Returns `PhaseLockError::InvalidPhase` if phase difference is NaN.
    pub fn new(entity_a: u64, entity_b: u64, phase_diff: f64) -> Result<Self, PhaseLockError> {
        if phase_diff.is_nan() {
            return Err(PhaseLockError::InvalidPhase { phase: phase_diff });
        }
        let normalized = phase_diff % Self::TAU;
        let normalized = if normalized < 0.0 {
            normalized + Self::TAU
        } else {
            normalized
        };
        Ok(Self {
            entity_a,
            entity_b,
            phase_difference: normalized,
            locked: false,
        })
    }

    /// Returns whether the two entities are phase-locked.
    pub fn is_locked(&self) -> bool {
        self.locked
    }

    /// Attempts to acquire phase lock.
    ///
    /// Lock is acquired if the phase difference is below the threshold.
    pub fn acquire_lock(&mut self) -> bool {
        if self.phase_difference < Self::LOCK_THRESHOLD {
            self.locked = true;
            true
        } else {
            false
        }
    }

    /// Releases the phase lock.
    pub fn release_lock(&mut self) {
        self.locked = false;
    }

    /// Updates the phase difference and checks for lock status.
    ///
    /// Returns whether lock was acquired or lost after the update.
    pub fn update_phase(&mut self, new_phase_diff: f64) -> Result<bool, PhaseLockError> {
        if new_phase_diff.is_nan() {
            return Err(PhaseLockError::InvalidPhase { phase: new_phase_diff });
        }
        let normalized = new_phase_diff % Self::TAU;
        let normalized = if normalized < 0.0 {
            normalized + Self::TAU
        } else {
            normalized
        };
        self.phase_difference = normalized;
        let was_locked = self.locked;
        self.locked = self.phase_difference < Self::LOCK_THRESHOLD;
        Ok(!was_locked && self.locked)
    }

    /// Returns the phase difference normalized to [-π, π].
    pub fn signed_phase_difference(&self) -> f64 {
        let mut diff = self.phase_difference;
        if diff > std::f64::consts::PI {
            diff -= Self::TAU;
        }
        diff
    }

    /// Returns the entity pair.
    pub fn entities(&self) -> (u64, u64) {
        (self.entity_a, self.entity_b)
    }

    /// Checks whether this lock involves the given entity.
    pub fn involves(&self, entity: u64) -> bool {
        self.entity_a == entity || self.entity_b == entity
    }

    /// Computes the coupling compatibility between two phase locks.
    pub fn compatibility(&self, other: &PhaseLock) -> f64 {
        let diff = (self.signed_phase_difference() - other.signed_phase_difference()).abs();
        1.0 - diff / (Self::TAU * 0.5)
    }
}

impl Default for PhaseLock {
    fn default() -> Self {
        Self {
            entity_a: 0,
            entity_b: 0,
            phase_difference: 0.0,
            locked: false,
        }
    }
}

/// Error type for phase lock failures.
#[derive(Debug, Clone, PartialEq)]
pub enum PhaseLockError {
    /// The phase difference is NaN.
    InvalidPhase { phase: f64 },
}

impl std::fmt::Display for PhaseLockError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PhaseLockError::InvalidPhase { phase } => {
                write!(f, "Invalid phase difference {}: must be a valid number", phase)
            }
        }
    }
}

impl std::error::Error for PhaseLockError {}