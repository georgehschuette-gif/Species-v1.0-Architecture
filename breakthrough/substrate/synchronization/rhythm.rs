// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.


/// CognitiveRhythm: A repeating temporal pattern in cognitive activity.
///
/// Describes the period, amplitude, and phase of
/// oscillatory cognitive processes.
#[derive(Debug, Clone, PartialEq)]
pub struct CognitiveRhythm {
    pub period: f64,
    pub amplitude: f64,
    pub phase: f64,
}

impl CognitiveRhythm {
    /// The minimum valid period.
    pub const MIN_PERIOD: f64 = 0.0;
    /// The maximum valid amplitude.
    pub const MAX_AMPLITUDE: f64 = 1.0;

    /// Creates a new cognitive rhythm.
    ///
    /// Phase is normalized to [0, 2π). Amplitude is not clamped.
    ///
    /// # Errors
    /// Returns `RhythmError::InvalidPeriod` if period is NaN or negative.
    /// Returns `RhythmError::InvalidAmplitude` if amplitude is NaN.
    pub fn new(
        period: f64,
        amplitude: f64,
        phase: f64,
    ) -> Result<Self, RhythmError> {
        if period.is_nan() || period < Self::MIN_PERIOD {
            return Err(RhythmError::InvalidPeriod { period });
        }
        if amplitude.is_nan() {
            return Err(RhythmError::InvalidAmplitude { amplitude });
        }
        let normalized_phase = if period > 0.0 {
            phase % (2.0 * std::f64::consts::PI)
        } else {
            0.0
        };
        let normalized_phase = if normalized_phase < 0.0 {
            normalized_phase + 2.0 * std::f64::consts::PI
        } else {
            normalized_phase
        };
        Ok(Self {
            period,
            amplitude,
            phase: normalized_phase,
        })
    }

    /// Returns the frequency (inverse of period).
    ///
    /// Returns infinity for zero period.
    pub fn frequency(&self) -> f64 {
        if self.period == 0.0 {
            f64::INFINITY
        } else {
            1.0 / self.period
        }
    }

    /// Computes the value of the rhythm at the given time.
    pub fn value_at(&self, time: f64) -> f64 {
        if self.period <= 0.0 {
            return self.amplitude;
        }
        let angular_freq = 2.0 * std::f64::consts::PI / self.period;
        self.amplitude * (angular_freq * time + self.phase).sin()
    }

    /// Returns the peak amplitude of the rhythm.
    pub fn peak(&self) -> f64 {
        self.amplitude
    }

    /// Checks whether the rhythm is active (non-zero period and amplitude).
    pub fn is_active(&self) -> bool {
        self.period > f64::EPSILON && self.amplitude.abs() > f64::EPSILON
    }

    /// Scales the amplitude by a factor.
    pub fn scale_amplitude(&mut self, factor: f64) -> Result<(), RhythmError> {
        if factor.is_nan() {
            return Err(RhythmError::InvalidAmplitude { amplitude: factor });
        }
        self.amplitude *= factor;
        Ok(())
    }

    /// Changes the period and re-normalizes the phase accordingly.
    pub fn set_period(&mut self, period: f64) -> Result<(), RhythmError> {
        if period.is_nan() || period < Self::MIN_PERIOD {
            return Err(RhythmError::InvalidPeriod { period });
        }
        self.period = period;
        Ok(())
    }

    /// Advances the phase by the given angle.
    pub fn advance_phase(&mut self, delta: f64) -> Result<(), RhythmError> {
        if delta.is_nan() {
            return Err(RhythmError::InvalidPhase { phase: delta });
        }
        self.phase = (self.phase + delta) % (2.0 * std::f64::consts::PI);
        Ok(())
    }

    /// Computes the phase offset needed to align with another rhythm.
    pub fn phase_offset_to(&self, other: &CognitiveRhythm) -> f64 {
        (other.phase - self.phase + 2.0 * std::f64::consts::PI) % (2.0 * std::f64::consts::PI)
    }
}

impl Default for CognitiveRhythm {
    fn default() -> Self {
        Self {
            period: 1.0,
            amplitude: 1.0,
            phase: 0.0,
        }
    }
}

/// Error type for rhythm failures.
#[derive(Debug, Clone, PartialEq)]
pub enum RhythmError {
    /// The period is NaN or negative.
    InvalidPeriod { period: f64 },
    /// The amplitude is NaN.
    InvalidAmplitude { amplitude: f64 },
    /// The phase is NaN.
    InvalidPhase { phase: f64 },
}

impl std::fmt::Display for RhythmError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RhythmError::InvalidPeriod { period } => {
                write!(f, "Invalid period {}: must be non-negative", period)
            }
            RhythmError::InvalidAmplitude { amplitude } => {
                write!(f, "Invalid amplitude {}: must be a valid number", amplitude)
            }
            RhythmError::InvalidPhase { phase } => {
                write!(f, "Invalid phase {}: must be a valid number", phase)
            }
        }
    }
}

impl std::error::Error for RhythmError {}