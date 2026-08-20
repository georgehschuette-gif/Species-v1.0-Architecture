// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.


/// TemporalArrow: The directionality of time in the cognitive ecosystem.
///
/// Ensures that cognitive processes are irreversible and ordered.
/// Provides methods to verify temporal ordering, causality, and
/// the consistency of sequential operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TemporalArrow;

impl TemporalArrow {
    /// Creates a new `TemporalArrow`.
    pub fn new() -> Self {
        Self
    }

    /// Checks whether `t1` precedes or equals `t2` (i.e., time flows forward).
    ///
    /// Uses an epsilon tolerance for equality.
    pub fn is_forward(&self, t1: f64, t2: f64) -> bool {
        if t1.is_nan() || t2.is_nan() {
            return false;
        }
        t1 <= t2 + f64::EPSILON
    }

    /// Checks whether `t1` strictly precedes `t2` (i.e., time has advanced).
    pub fn is_strictly_forward(&self, t1: f64, t2: f64) -> bool {
        if t1.is_nan() || t2.is_nan() {
            return false;
        }
        t1 < t2 - f64::EPSILON
    }

    /// Checks whether two timestamps are simultaneous.
    pub fn is_simultaneous(&self, t1: f64, t2: f64) -> bool {
        if t1.is_nan() || t2.is_nan() {
            return false;
        }
        (t1 - t2).abs() < f64::EPSILON
    }

    /// Returns the ordering of two timestamps.
    ///
    /// Returns `None` if either timestamp is NaN.
    pub fn compare(&self, t1: f64, t2: f64) -> Option<std::cmp::Ordering> {
        if t1.is_nan() || t2.is_nan() {
            return None;
        }
        // Use total cmp for deterministic ordering of all f64 values
        Some(t1.partial_cmp(&t2).unwrap_or(std::cmp::Ordering::Equal))
    }

    /// Checks whether the sequence of timestamps is monotonically non-decreasing.
    pub fn is_sequence_valid(&self, timestamps: &[f64]) -> bool {
        for pair in timestamps.windows(2) {
            if !self.is_forward(pair[0], pair[1]) {
                return false;
            }
        }
        true
    }

    /// Returns the duration between two points if time flows forward.
    ///
    /// Returns `None` if `t2` precedes `t1` or either is NaN.
    pub fn duration(&self, t1: f64, t2: f64) -> Option<f64> {
        if !self.is_forward(t1, t2) {
            return None;
        }
        Some(t2 - t1)
    }

    /// Validates that a sequence of events maintains causal consistency.
    ///
    /// Each event's timestamp must not precede the previous event's timestamp.
    pub fn validate_causality(&self, timestamps: &[f64]) -> Result<(), ArrowError> {
        for (i, pair) in timestamps.windows(2).enumerate() {
            if !self.is_forward(pair[0], pair[1]) {
                return Err(ArrowError::TemporalInversion {
                    index: i,
                    earlier: pair[1],
                    later: pair[0],
                });
            }
        }
        Ok(())
    }
}

impl Default for TemporalArrow {
    fn default() -> Self {
        Self
    }
}

/// Error type for temporal arrow validation failures.
#[derive(Debug, Clone, PartialEq)]
pub enum ArrowError {
    /// Timestamps violate monotonic ordering at the given index.
    TemporalInversion { index: usize, earlier: f64, later: f64 },
}

impl std::fmt::Display for ArrowError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArrowError::TemporalInversion {
                index,
                earlier,
                later,
            } => {
                write!(
                    f,
                    "Temporal inversion at event {}: {} precedes {}",
                    index, earlier, later
                )
            }
        }
    }
}

impl std::error::Error for ArrowError {}