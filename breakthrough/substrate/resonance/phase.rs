// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.


/// PhaseAlignment: The synchronization of oscillatory phases between entities.
///
/// Represents where an entity is in its oscillation cycle at a given moment.
/// Phase is always normalized to [0, 2π) and coherence is in [0, 1].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PhaseAlignment {
    pub phase: f64,
    pub coherence: f64,
}

impl PhaseAlignment {
    /// Full circle in radians.
    pub const TAU: f64 = 2.0 * std::f64::consts::PI;
    /// Minimum coherence for a meaningful phase relationship.
    pub const MIN_COHERENCE: f64 = 0.0;
    /// Maximum coherence value.
    pub const MAX_COHERENCE: f64 = 1.0;

    /// Creates a new phase alignment.
    ///
    /// Phase is normalized to [0, 2π). Coherence is clamped to [0, 1].
    ///
    /// # Errors
    /// Returns `PhaseError::InvalidCoherence` if coherence is NaN or outside [0, 1].
    pub fn new(phase: f64, coherence: f64) -> Result<Self, PhaseError> {
        if coherence.is_nan() || coherence < Self::MIN_COHERENCE || coherence > Self::MAX_COHERENCE {
            return Err(PhaseError::InvalidCoherence { coherence });
        }
        let normalized_phase = phase % Self::TAU;
        let normalized_phase = if normalized_phase < 0.0 {
            normalized_phase + Self::TAU
        } else {
            normalized_phase
        };
        Ok(Self {
            phase: normalized_phase,
            coherence,
        })
    }

    /// Creates a phase alignment at zero phase with zero coherence.
    pub fn zero() -> Self {
        Self {
            phase: 0.0,
            coherence: 0.0,
        }
    }

    /// Returns the phase difference between two alignments.
    ///
    /// The result is always in the range [0, π].
    pub fn phase_difference(&self, other: &PhaseAlignment) -> f64 {
        let diff = (self.phase - other.phase).abs();
        if diff > std::f64::consts::PI {
            Self::TAU - diff
        } else {
            diff
        }
    }

    /// Returns whether two phases are approximately aligned.
    pub fn is_aligned_with(&self, other: &PhaseAlignment, tolerance: f64) -> bool {
        self.phase_difference(other) < tolerance
    }

    /// Computes the interaction strength between two phase alignments.
    ///
    /// This is the product of their coherences modulated by their phase proximity.
    pub fn interaction_strength(&self, other: &PhaseAlignment) -> f64 {
        let coherence_product = self.coherence * other.coherence;
        let phase_proximity = 1.0 - self.phase_difference(other) / std::f64::consts::PI;
        coherence_product * phase_proximity
    }

    /// Advances the phase by the given angle, wrapping around.
    pub fn advance(&self, delta: f64) -> Result<Self, PhaseError> {
        let new_phase = self.phase + delta;
        Self::new(new_phase, self.coherence)
    }

    /// Inverts the phase (shifts by π radians).
    pub fn invert(&self) -> Result<Self, PhaseError> {
        Self::new(self.phase + std::f64::consts::PI, self.coherence)
    }
}

impl Default for PhaseAlignment {
    fn default() -> Self {
        Self::zero()
    }
}

/// Error type for phase validation failures.
#[derive(Debug, Clone, PartialEq)]
pub enum PhaseError {
    /// The coherence value is outside [0, 1] or NaN.
    InvalidCoherence { coherence: f64 },
}

impl std::fmt::Display for PhaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PhaseError::InvalidCoherence { coherence } => {
                write!(f, "Invalid coherence {}: must be in [0.0, 1.0]", coherence)
            }
        }
    }
}

impl std::error::Error for PhaseError {}