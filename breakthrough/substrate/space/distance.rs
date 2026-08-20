// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.


/// Distance: The measure of separation between two positions in cognitive space.
///
/// Internally guaranteed to be non-negative; negative inputs are clamped to zero.
/// Uses an epsilon tolerance of `1e-9` for zero comparisons.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Distance(pub f64);

impl Distance {
    /// The smallest distance that is considered non-zero.
    pub const EPSILON: f64 = 1e-9;

    /// Creates a new `Distance` from the given value.
    ///
    /// Negative values are clamped to `0.0`.
    /// NaN values are treated as `0.0`.
    ///
    /// # Errors
    /// Returns `DistanceError::NaNValue` if the input is NaN.
    pub fn new(value: f64) -> Result<Self, DistanceError> {
        if value.is_nan() {
            return Err(DistanceError::NaNValue);
        }
        Ok(Self(value.max(0.0)))
    }

    /// Creates a new `Distance` without validation.
    ///
    /// # Safety
    /// The caller must ensure that `value` is non-negative and finite.
    pub const fn new_unchecked(value: f64) -> Self {
        Self(value.max(0.0))
    }

    /// Returns whether this distance is effectively zero.
    pub fn is_zero(&self) -> bool {
        self.0 < Self::EPSILON
    }

    /// Returns whether this distance is strictly positive.
    pub fn is_positive(&self) -> bool {
        self.0 >= Self::EPSILON
    }

    /// Returns the squared distance.
    ///
    /// Useful for comparisons where the square root is unnecessary.
    pub fn squared(&self) -> f64 {
        self.0 * self.0
    }

    /// Computes the midpoint between two distances along a linear scale.
    pub fn midpoint(&self, other: &Distance) -> Self {
        Self((self.0 + other.0) * 0.5)
    }

    /// Returns the sum of two distances.
    pub fn add(&self, other: &Distance) -> Result<Self, DistanceError> {
        let sum = self.0 + other.0;
        if sum.is_infinite() {
            return Err(DistanceError::Overflow);
        }
        Ok(Self(sum))
    }

    /// Returns the difference of two distances, clamped at zero.
    pub fn sub(&self, other: &Distance) -> Self {
        Self((self.0 - other.0).max(0.0))
    }

    /// Scales the distance by a given factor.
    ///
    /// Negative factors are treated as their absolute value.
    pub fn scale(&self, factor: f64) -> Result<Self, DistanceError> {
        if factor.is_nan() {
            return Err(DistanceError::NaNValue);
        }
        let scaled = self.0 * factor.abs();
        if scaled.is_infinite() {
            return Err(DistanceError::Overflow);
        }
        Ok(Self(scaled))
    }

    /// Returns the ratio of this distance to another.
    ///
    /// Returns `None` if `other` is zero.
    pub fn ratio(&self, other: &Distance) -> Option<f64> {
        if other.is_zero() {
            None
        } else {
            Some(self.0 / other.0)
        }
    }
}

impl Default for Distance {
    fn default() -> Self {
        Self(0.0)
    }
}

impl std::ops::Add for Distance {
    type Output = Result<Distance, DistanceError>;

    fn add(self, rhs: Self) -> Self::Output {
        let sum = self.0 + rhs.0;
        if sum.is_infinite() {
            return Err(DistanceError::Overflow);
        }
        Ok(Distance(sum))
    }
}

impl std::ops::Sub for Distance {
    type Output = Distance;

    fn sub(self, rhs: Self) -> Self::Output {
        Distance((self.0 - rhs.0).max(0.0))
    }
}

impl std::ops::Mul<f64> for Distance {
    type Output = Result<Distance, DistanceError>;

    fn mul(self, rhs: f64) -> Self::Output {
        self.scale(rhs)
    }
}

impl std::ops::Div<f64> for Distance {
    type Output = Result<Distance, DistanceError>;

    fn div(self, rhs: f64) -> Self::Output {
        if rhs.abs() < f64::EPSILON {
            return Err(DistanceError::DivisionByZero);
        }
        self.scale(1.0 / rhs)
    }
}

/// Error type for distance validation failures.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DistanceError {
    /// The input value was NaN.
    NaNValue,
    /// The result would overflow to infinity.
    Overflow,
    /// Division by zero.
    DivisionByZero,
}

impl std::fmt::Display for DistanceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DistanceError::NaNValue => write!(f, "Distance value cannot be NaN"),
            DistanceError::Overflow => write!(f, "Distance computation overflowed"),
            DistanceError::DivisionByZero => write!(f, "Division by zero in distance computation"),
        }
    }
}

impl std::error::Error for DistanceError {}