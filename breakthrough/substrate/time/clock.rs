// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.


/// CognitiveClock: The timekeeping mechanism for the cognitive ecosystem.
///
/// Tracks the current time and the most recent delta-tick.
/// Time advances monotonically; negative deltas are rejected.
///
/// # Invariants
/// - `time` is always non-negative after any successful tick
/// - `delta` reflects the most recent successful tick
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CognitiveClock {
    pub time: f64,
    pub delta: f64,
}

impl CognitiveClock {
    /// Creates a new clock starting at time zero with no delta elapsed.
    pub fn new() -> Self {
        Self {
            time: 0.0,
            delta: 0.0,
        }
    }

    /// Advances the clock by the given delta.
    ///
    /// # Errors
    /// Returns `ClockError::NegativeDelta` if `delta` is negative.
    /// Returns `ClockError::Overflow` if the resulting time would overflow.
    pub fn tick(&mut self, delta: f64) -> Result<(), ClockError> {
        if delta.is_nan() {
            return Err(ClockError::NaNValue);
        }
        if delta < 0.0 {
            return Err(ClockError::NegativeDelta { delta });
        }
        let new_time = self.time + delta;
        if new_time.is_infinite() {
            return Err(ClockError::Overflow);
        }
        self.delta = delta;
        self.time = new_time;
        Ok(())
    }

    /// Resets the clock to time zero with zero delta.
    pub fn reset(&mut self) {
        self.time = 0.0;
        self.delta = 0.0;
    }

    /// Returns the current time.
    pub fn now(&self) -> f64 {
        self.time
    }

    /// Returns the most recent delta-tick.
    pub fn last_delta(&self) -> f64 {
        self.delta
    }

    /// Returns the time at which a future tick of `delta` would occur,
    /// without actually advancing the clock.
    pub fn peek_tick(&self, delta: f64) -> Result<f64, ClockError> {
        if delta.is_nan() {
            return Err(ClockError::NaNValue);
        }
        if delta < 0.0 {
            return Err(ClockError::NegativeDelta { delta });
        }
        let future = self.time + delta;
        if future.is_infinite() {
            return Err(ClockError::Overflow);
        }
        Ok(future)
    }

    /// Checks whether the clock has elapsed at least `target` time.
    pub fn has_elapsed(&self, target: f64) -> bool {
        self.time >= target && target.is_finite()
    }
}

impl Default for CognitiveClock {
    fn default() -> Self {
        Self::new()
    }
}

/// Error type for clock operation failures.
#[derive(Debug, Clone, PartialEq)]
pub enum ClockError {
    /// The delta value is NaN.
    NaNValue,
    /// The delta is negative, violating monotonic time.
    NegativeDelta { delta: f64 },
    /// The resulting time would overflow to infinity.
    Overflow,
}

impl std::fmt::Display for ClockError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClockError::NaNValue => write!(f, "Delta value cannot be NaN"),
            ClockError::NegativeDelta { delta } => {
                write!(f, "Negative delta {} violates monotonic time", delta)
            }
            ClockError::Overflow => write!(f, "Clock time would overflow"),
        }
    }
}

impl std::error::Error for ClockError {}