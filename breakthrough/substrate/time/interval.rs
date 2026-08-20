// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.


/// TimeInterval: A duration between two temporal points.
///
/// Guaranteed to be non-negative. NaN inputs are treated as zero.
/// Uses an epsilon tolerance for zero comparisons.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct TimeInterval(pub f64);

impl TimeInterval {
    /// The smallest interval that is considered non-zero.
    pub const EPSILON: f64 = 1e-9;

    /// Creates a new `TimeInterval` from the given duration.
    ///
    /// Negative values are clamped to zero. NaN values are treated as zero.
    ///
    /// # Errors
    /// Returns `IntervalError::InfiniteValue` if the duration is infinite.
    pub fn new(duration: f64) -> Result<Self, IntervalError> {
        if duration.is_nan() {
            return Err(IntervalError::NaNValue);
        }
        if duration.is_infinite() {
            return Err(IntervalError::InfiniteValue);
        }
        Ok(Self(duration.max(0.0)))
    }

    /// Creates a new `TimeInterval` without validation.
    ///
    /// # Safety
    /// The caller must ensure that `duration` is non-negative and finite.
    pub const fn new_unchecked(duration: f64) -> Self {
        Self(duration.max(0.0))
    }

    /// Returns whether this interval is effectively zero.
    pub fn is_zero(&self) -> bool {
        self.0 < Self::EPSILON
    }

    /// Returns whether this interval is strictly positive.
    pub fn is_positive(&self) -> bool {
        self.0 >= Self::EPSILON
    }

    /// Returns half of this interval.
    pub fn half(&self) -> Self {
        Self(self.0 * 0.5)
    }

    /// Returns the double of this interval.
    pub fn doubled(&self) -> Result<Self, IntervalError> {
        let doubled = self.0 * 2.0;
        if doubled.is_infinite() {
            return Err(IntervalError::Overflow);
        }
        Ok(Self(doubled))
    }

    /// Adds another interval to this one.
    pub fn add(&self, other: &TimeInterval) -> Result<Self, IntervalError> {
        let sum = self.0 + other.0;
        if sum.is_infinite() {
            return Err(IntervalError::Overflow);
        }
        Ok(Self(sum))
    }

    /// Subtracts another interval from this one, clamped at zero.
    pub fn sub(&self, other: &TimeInterval) -> Self {
        Self((self.0 - other.0).max(0.0))
    }

    /// Scales the interval by a given factor.
    pub fn scale(&self, factor: f64) -> Result<Self, IntervalError> {
        if factor.is_nan() {
            return Err(IntervalError::NaNValue);
        }
        let scaled = self.0 * factor;
        if scaled.is_infinite() {
            return Err(IntervalError::Overflow);
        }
        Ok(Self(scaled))
    }

    /// Checks whether this interval fully contains another.
    pub fn contains(&self, other: &TimeInterval) -> bool {
        self.0 >= other.0
    }

    /// Returns the ratio of this interval to another.
    ///
    /// Returns `None` if `other` is zero.
    pub fn ratio(&self, other: &TimeInterval) -> Option<f64> {
        if other.is_zero() {
            None
        } else {
            Some(self.0 / other.0)
        }
    }
}

impl Default for TimeInterval {
    fn default() -> Self {
        Self(0.0)
    }
}

impl std::ops::Add for TimeInterval {
    type Output = Result<TimeInterval, IntervalError>;

    fn add(self, rhs: Self) -> Self::Output {
        let sum = self.0 + rhs.0;
        if sum.is_infinite() {
            return Err(IntervalError::Overflow);
        }
        Ok(TimeInterval(sum))
    }
}

impl std::ops::Sub for TimeInterval {
    type Output = TimeInterval;

    fn sub(self, rhs: Self) -> Self::Output {
        TimeInterval((self.0 - rhs.0).max(0.0))
    }
}

impl std::ops::Mul<f64> for TimeInterval {
    type Output = Result<TimeInterval, IntervalError>;

    fn mul(self, rhs: f64) -> Self::Output {
        self.scale(rhs)
    }
}

impl std::ops::Div<f64> for TimeInterval {
    type Output = Result<TimeInterval, IntervalError>;

    fn div(self, rhs: f64) -> Self::Output {
        if rhs.abs() < f64::EPSILON {
            return Err(IntervalError::DivisionByZero);
        }
        self.scale(1.0 / rhs)
    }
}

/// Error type for interval validation failures.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IntervalError {
    /// The input value is NaN.
    NaNValue,
    /// The input value is infinite.
    InfiniteValue,
    /// The result would overflow to infinity.
    Overflow,
    /// Division by zero.
    DivisionByZero,
}

impl std::fmt::Display for IntervalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IntervalError::NaNValue => write!(f, "Interval value cannot be NaN"),
            IntervalError::InfiniteValue => write!(f, "Interval value cannot be infinite"),
            IntervalError::Overflow => write!(f, "Interval computation overflowed"),
            IntervalError::DivisionByZero => write!(f, "Division by zero in interval computation"),
        }
    }
}

impl std::error::Error for IntervalError {}